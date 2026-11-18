// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::collections::{BTreeMap, VecDeque};
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use futures::{
    FutureExt,
    future::poll_fn,
    task::{ArcWake, waker_ref},
};
use hyperlight_host::{HyperlightError, func::Registerable};

use crate::{Error, Result, async_calls, errno, hostnet};

#[derive(Default)]
pub(crate) struct WakeSet(Mutex<Vec<Weak<Mutex<Option<Waker>>>>>);

pub(crate) struct Subscription(Arc<Mutex<Option<Waker>>>);

impl Subscription {
    pub fn register(&self, waker: &Waker) {
        *self.0.lock().unwrap() = Some(waker.clone());
    }
}

impl WakeSet {
    pub fn subscribe(&self) -> Subscription {
        let mut waiters = self.0.lock().unwrap();
        waiters.retain(|entry| entry.strong_count() != 0);
        let entry = Arc::new(Mutex::new(None));
        waiters.push(Arc::downgrade(&entry));
        Subscription(entry)
    }

    pub fn notify(&self) {
        let waiters: Vec<_> = self
            .0
            .lock()
            .unwrap()
            .iter()
            .filter_map(Weak::upgrade)
            .collect();
        for waker in waiters {
            let waker = waker.lock().unwrap().take();
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }

    pub fn with_context<T>(self: &Arc<Self>, poll: impl FnOnce(&mut Context<'_>) -> T) -> T {
        let waker = waker_ref(self);
        poll(&mut Context::from_waker(&waker))
    }
}

impl ArcWake for WakeSet {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.notify();
    }
}

pub(crate) struct Execution {
    pub id: u64,
    pub command: CommandMetadata,
    pub driving: Mutex<DrivingState>,
    pub needs_restore: AtomicBool,
    pub kill_requested: AtomicBool,
    pub failure: Mutex<Option<String>>,
    pub done: Arc<AtomicBool>,
    pub process: Mutex<ProcessState>,
    pub pipes: [Arc<Pipe>; 3],
    pub inbox: Arc<Mutex<async_calls::Inbox>>,
    pub calls: Mutex<async_calls::PendingCalls>,
    pub net: Arc<hostnet::Net>,
    pub wake: Arc<WakeSet>,
    pub deadline: Mutex<Option<Instant>>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct CommandMetadata {
    pub name: Option<String>,
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: BTreeMap<String, String>,
    pub stdio: [crate::command::Stdio; 3],
}

pub(crate) struct DrivingState {
    pub parked: Arc<AtomicBool>,
    pub generation: u64,
    pub streams: [bool; 3],
    pub resume: Option<Vec<u8>>,
    pub checkpoint_ready: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ExecutionSnapshot {
    pub command_only: bool,
    next: u64,
    next_request: u64,
    entries: Vec<ExecutionImage>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct ExecutionImage {
    id: u64,
    command: CommandMetadata,
    done: bool,
    failure: Option<String>,
    process: ProcessState,
    pipes: [PipeState; 3],
    streams: [bool; 3],
}

impl ExecutionSnapshot {
    pub(crate) fn validate(&self) -> crate::Result<()> {
        let mut ids = std::collections::BTreeSet::new();
        let mut names = std::collections::BTreeSet::new();
        if !self.command_only {
            return Err(hyperlight_host::new_error!("snapshot is not command-only").into());
        }
        for entry in &self.entries {
            if entry.id == 0 || entry.id > self.next || !ids.insert(entry.id) {
                return Err(hyperlight_host::new_error!("invalid snapshot execution ID").into());
            }
            if let Some(name) = &entry.command.name
                && (name.is_empty() || name.contains('\0') || !names.insert(name))
            {
                return Err(hyperlight_host::new_error!("invalid snapshot command name").into());
            }
            if entry.pipes.iter().any(|pipe| !pipe.bytes.is_empty())
                || entry.pipes[0].guest_bytes > PIPE_CAPACITY
                || (entry.pipes[0].guest_eof && !entry.pipes[0].eof && !entry.pipes[0].closed)
                || entry.pipes[1..]
                    .iter()
                    .any(|pipe| pipe.guest_bytes != 0 || pipe.guest_received != 0 || pipe.guest_eof)
            {
                return Err(hyperlight_host::new_error!("invalid snapshot pipe state").into());
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct Executions {
    pub supported: AtomicBool,
    pub command_only: AtomicBool,
    current: AtomicU64,
    active: AtomicU64,
    next: Mutex<u64>,
    entries: Mutex<BTreeMap<u64, Arc<Execution>>>,
    completed: Mutex<Vec<Arc<Execution>>>,
}

impl Executions {
    pub fn create(
        &self,
        inbox: &Arc<Mutex<async_calls::Inbox>>,
        net: Option<&Arc<hostnet::Net>>,
        streams: [bool; 3],
        command: CommandMetadata,
    ) -> crate::Result<(u64, Arc<Execution>)> {
        let mut entries = self.entries.lock().unwrap();
        if let Some(name) = &command.name {
            if name.is_empty() || name.contains('\0') {
                return Err(Error::Wait(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "command name must be nonempty and contain no NUL",
                )));
            }
            if entries
                .values()
                .any(|entry| entry.command.name.as_ref() == Some(name))
            {
                return Err(Error::Wait(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "command name is already in use",
                )));
            }
        }
        if entries
            .values()
            .filter(|entry| !entry.done.load(Ordering::Relaxed))
            .count()
            >= 1024
        {
            return Err(HyperlightError::Error("too many command executions".into()).into());
        }
        let mut next = self.next.lock().unwrap();
        let id = next
            .checked_add(1)
            .ok_or_else(|| HyperlightError::Error("execution IDs exhausted".into()))?;
        *next = id;
        let entry = Arc::new(Execution::new(id, inbox, net, streams, command));
        entries.insert(id, entry.clone());
        Ok((id, entry))
    }

    pub fn get(&self, id: u64) -> Option<Arc<Execution>> {
        self.entries.lock().unwrap().get(&id).cloned()
    }

    pub fn list(&self) -> Vec<Arc<Execution>> {
        self.entries.lock().unwrap().values().cloned().collect()
    }

    pub fn complete_entry(&self) {
        // Detach under the VM lock, but drop futures and notify waiters only
        // after leaving it: embedder destructors and wakers may reenter.
        let mut entries = self.entries.lock().unwrap();
        let mut completed = self.completed.lock().unwrap();
        entries.retain(|_, entry| {
            if entry.done.load(Ordering::Relaxed) {
                completed.push(entry.clone());
                false
            } else {
                true
            }
        });
    }

    pub fn prepare_restore(&self) -> crate::Result<()> {
        let entries: Vec<_> = self.entries.lock().unwrap().values().cloned().collect();
        if entries
            .iter()
            .any(|entry| !entry.driving.lock().unwrap().parked.load(Ordering::Relaxed))
        {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::InvalidInput,
                "park or drop all child handles before restore",
            )));
        }
        for entry in entries {
            let mut calls = match entry.calls.try_lock() {
                Ok(calls) => calls,
                Err(std::sync::TryLockError::WouldBlock) => {
                    return Err(Error::Wait(io::Error::new(
                        io::ErrorKind::WouldBlock,
                        "a command poll is still parking",
                    )));
                }
                Err(std::sync::TryLockError::Poisoned(error)) => panic!("{error}"),
            };
            entry.park(&mut calls);
        }
        Ok(())
    }

    pub fn snapshot(
        &self,
        inbox: &Arc<Mutex<async_calls::Inbox>>,
        sandbox: &mut crate::AppSandbox,
    ) -> crate::Result<(ExecutionSnapshot, Arc<crate::Snapshot>)> {
        let entries = self.list();
        // Hold every engine and stream-access gate through the guest capture.
        // The caller holds the VM and lifecycle locks; never wait for an active poll.
        let mut calls = Vec::with_capacity(entries.len());
        let mut driving = Vec::with_capacity(entries.len());
        for entry in &entries {
            calls.push(match entry.calls.try_lock() {
                Ok(calls) => calls,
                Err(std::sync::TryLockError::WouldBlock) => {
                    return Err(Error::Wait(io::Error::new(
                        io::ErrorKind::WouldBlock,
                        "a command is still being driven or parked",
                    )));
                }
                Err(std::sync::TryLockError::Poisoned(error)) => panic!("{error}"),
            });
            driving.push(match entry.driving.try_lock() {
                Ok(state) => state,
                Err(std::sync::TryLockError::WouldBlock) => {
                    return Err(Error::Wait(io::Error::new(
                        io::ErrorKind::WouldBlock,
                        "a command handle or pipe operation is in progress",
                    )));
                }
                Err(std::sync::TryLockError::Poisoned(error)) => panic!("{error}"),
            });
        }
        if entries
            .iter()
            .any(|entry| entry.kill_requested.load(Ordering::Relaxed))
        {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::WouldBlock,
                "a dropped command is awaiting termination",
            )));
        }
        let retained: Vec<_> = calls
            .iter()
            .zip(&driving)
            .map(|(calls, state)| {
                state
                    .resume
                    .clone()
                    .unwrap_or_else(|| calls.snapshot_batch())
            })
            .collect();
        let mut replies: Vec<_> = retained
            .iter()
            .flat_map(|batch| async_calls::batch_records(batch))
            .collect();
        replies.sort_unstable_by_key(|(id, _)| *id);
        checkpoint_inputs(&entries, &replies, |batch, ids| {
            sandbox.checkpoint_command_inputs(batch)?;
            for (calls, state) in calls.iter_mut().zip(&mut driving) {
                if calls.checkpoint_delivered(ids) {
                    state.checkpoint_ready = true;
                }
                if let Some(resume) = &mut state.resume {
                    async_calls::acknowledge_batch(resume, ids);
                }
            }
            Ok(())
        })?;
        let guest = sandbox.snapshot()?;
        let mut images = Vec::with_capacity(entries.len());
        for (entry, state) in entries.iter().zip(&driving) {
            if !entry.pipes[0].state.lock().unwrap().bytes.is_empty() {
                return Err(hyperlight_host::new_error!(
                    "guest did not checkpoint all pending stdin"
                )
                .into());
            }
            if entry.kill_requested.load(Ordering::Relaxed) {
                return Err(Error::Wait(io::Error::new(
                    io::ErrorKind::WouldBlock,
                    "a dropped command is awaiting termination",
                )));
            }
            images.push(ExecutionImage {
                id: entry.id,
                command: entry.command.clone(),
                done: entry.done.load(Ordering::Relaxed),
                failure: entry.failure.lock().unwrap().clone(),
                process: entry.process.lock().unwrap().clone(),
                pipes: std::array::from_fn(|i| {
                    let pipe = entry.pipes[i].state.lock().unwrap();
                    PipeState {
                        // Input is in guest memory; output belongs to the reader.
                        bytes: VecDeque::new(),
                        guest_bytes: pipe.guest_bytes,
                        guest_received: pipe.guest_received,
                        guest_eof: pipe.guest_eof,
                        eof: pipe.eof,
                        closed: pipe.closed,
                        mode: pipe.mode,
                        dropped: pipe.dropped,
                    }
                }),
                streams: state.streams,
            });
        }
        let executions = ExecutionSnapshot {
            command_only: self.command_only.load(Ordering::Relaxed),
            next: *self.next.lock().unwrap(),
            next_request: inbox.lock().unwrap().next_id(),
            entries: images,
        };
        Ok((executions, guest))
    }

    pub fn restore(
        &self,
        snapshot: &ExecutionSnapshot,
        inbox: &Arc<Mutex<async_calls::Inbox>>,
        net: Option<&Arc<hostnet::Net>>,
    ) {
        self.command_only
            .store(snapshot.command_only, Ordering::Relaxed);
        let mut entries = BTreeMap::new();
        for image in &snapshot.entries {
            let mut entry =
                Execution::new(image.id, inbox, net, image.streams, image.command.clone());
            entry.done.store(image.done, Ordering::Relaxed);
            *entry.failure.get_mut().unwrap() = image.failure.clone();
            *entry.process.get_mut().unwrap() = image.process.clone();
            entry.pipes = std::array::from_fn(|i| {
                Arc::new(Pipe {
                    state: Mutex::new(image.pipes[i].clone()),
                    ..Pipe::default()
                })
            });
            let state = entry.driving.get_mut().unwrap();
            state.parked.store(true, Ordering::Relaxed);
            state.resume = Some(async_calls::empty_batch());
            entry.needs_restore.store(true, Ordering::Relaxed);
            entries.insert(image.id, Arc::new(entry));
        }
        *self.entries.lock().unwrap() = entries;
        let mut next = self.next.lock().unwrap();
        *next = (*next).max(snapshot.next);
        inbox.lock().unwrap().restore_next_id(snapshot.next_request);
        self.current.store(0, Ordering::Relaxed);
        self.active.store(0, Ordering::Relaxed);
    }

    pub fn negotiate(&self, inbox: &Arc<Mutex<async_calls::Inbox>>) {
        let inbox = inbox.lock().unwrap();
        for entry in self.entries.lock().unwrap().values() {
            let mut child = entry.inbox.lock().unwrap();
            child.negotiated = inbox.negotiated;
            child.interrupt_supported = inbox.interrupt_supported;
        }
    }

    pub fn current(&self) -> hyperlight_host::Result<Option<Arc<Execution>>> {
        let id = self.current.load(Ordering::Relaxed);
        if id == 0 {
            return Ok(None);
        }
        self.entries
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .map(Some)
            .ok_or_else(|| HyperlightError::Error(format!("unknown execution {id}")))
    }

    pub fn before_entry(&self, id: u64) {
        self.active.store(id, Ordering::Relaxed);
        if let Some(execution) = self.entries.lock().unwrap().get(&id) {
            *execution.deadline.lock().unwrap() = None;
        }
    }

    pub fn wake_all(&self) {
        let completed = std::mem::take(&mut *self.completed.lock().unwrap());
        for entry in completed {
            match entry.calls.try_lock() {
                Ok(mut calls) => calls.clear(),
                Err(std::sync::TryLockError::WouldBlock) => {}
                Err(std::sync::TryLockError::Poisoned(error)) => panic!("{error}"),
            }
            entry.net.reset();
            entry.pipes[0].close();
            entry.pipes[1].eof();
            entry.pipes[2].eof();
            entry.wake.notify();
        }
        let entries: Vec<_> = self.entries.lock().unwrap().values().cloned().collect();
        for entry in entries {
            if entry.done.load(Ordering::Relaxed) {
                match entry.calls.try_lock() {
                    Ok(mut calls) => calls.clear(),
                    Err(std::sync::TryLockError::WouldBlock) => {}
                    Err(std::sync::TryLockError::Poisoned(error)) => panic!("{error}"),
                }
            }
            entry.wake.notify();
        }
    }

    pub fn register(self: &Arc<Self>, target: &mut impl Registerable) -> crate::Result<()> {
        let registry = self.clone();
        target.register_host_function(
            "ExecutionEnter",
            move || -> hyperlight_host::Result<Vec<u8>> {
                let id = registry.active.load(Ordering::Relaxed);
                let restore = if id == 0 {
                    false
                } else {
                    registry
                        .get(id)
                        .ok_or_else(|| {
                            HyperlightError::Error("entering an unknown execution".into())
                        })?
                        .needs_restore
                        .swap(false, Ordering::Relaxed)
                };
                let mut reply = Vec::with_capacity(9);
                reply.extend_from_slice(&id.to_le_bytes());
                reply.push(u8::from(restore));
                Ok(reply)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "ExecutionEvents",
            move |bytes: Vec<u8>| -> hyperlight_host::Result<i32> {
                registry.events(&bytes)?;
                Ok(0)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "ExecutionRestore",
            move || -> hyperlight_host::Result<i32> {
                let id = registry.active.load(Ordering::Relaxed);
                let entry = registry
                    .entries
                    .lock()
                    .unwrap()
                    .get(&id)
                    .cloned()
                    .ok_or_else(|| {
                        HyperlightError::Error("restoring an unknown execution".into())
                    })?;
                Ok(i32::from(
                    entry.needs_restore.swap(false, Ordering::Relaxed),
                ))
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "ExecutionActive",
            move || -> hyperlight_host::Result<u64> { Ok(registry.active.load(Ordering::Relaxed)) },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "ExecutionDone",
            move || -> hyperlight_host::Result<i32> {
                let entry = registry.current()?.ok_or_else(|| {
                    HyperlightError::Error("execution completion without owner".into())
                })?;
                entry.done.store(true, Ordering::Relaxed);
                entry.wake.notify();
                Ok(0)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandEvent",
            move |kind: i32, value: i32| -> hyperlight_host::Result<i32> {
                registry
                    .current()?
                    .ok_or_else(|| {
                        HyperlightError::Error("command event without execution".into())
                    })?
                    .event(kind, value)?;
                Ok(0)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandIo",
            move |stream: u64, bytes: Vec<u8>| -> hyperlight_host::Result<Vec<u8>> {
                registry
                    .current()?
                    .ok_or_else(|| HyperlightError::Error("command I/O without execution".into()))?
                    .io(stream, bytes)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandStdin",
            move |id: u64, received: u64, unread: u64| -> hyperlight_host::Result<Vec<u8>> {
                registry
                    .get(id)
                    .ok_or_else(|| {
                        HyperlightError::Error("stdin consumption without execution".into())
                    })?
                    .pipes[0]
                    .consume_stdin(received, unread)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandRead",
            move |count: u64| -> hyperlight_host::Result<Vec<u8>> {
                registry
                    .current()?
                    .ok_or_else(|| HyperlightError::Error("command read without execution".into()))?
                    .stdio_read(count)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandWrite",
            move |stream: u64, bytes: Vec<u8>| -> hyperlight_host::Result<Vec<u8>> {
                registry
                    .current()?
                    .ok_or_else(|| {
                        HyperlightError::Error("command write without execution".into())
                    })?
                    .stdio_write(stream, &bytes)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandPoll",
            move |stream: u64| -> hyperlight_host::Result<Vec<u8>> {
                registry
                    .current()?
                    .ok_or_else(|| HyperlightError::Error("command poll without execution".into()))?
                    .stdio_poll(stream)
            },
        )?;
        target.register_host_function(
            "CommandServiceVersion",
            || -> hyperlight_host::Result<u64> { Ok(11) },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandServiceReady",
            move || -> hyperlight_host::Result<i32> {
                registry.supported.store(true, Ordering::Relaxed);
                Ok(0)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "CommandBootReady",
            move || -> hyperlight_host::Result<i32> {
                registry.command_only.store(true, Ordering::Relaxed);
                Ok(0)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "ExecutionContext",
            move |id: u64| -> hyperlight_host::Result<i32> {
                if id != 0 && !registry.entries.lock().unwrap().contains_key(&id) {
                    return Err(HyperlightError::Error(format!("unknown execution {id}")));
                }
                registry.current.store(id, Ordering::Relaxed);
                Ok(0)
            },
        )?;
        let registry = self.clone();
        target.register_host_function(
            "ExecutionTimers",
            move |timers: Vec<u8>| -> hyperlight_host::Result<i32> {
                let (records, tail) = timers.as_chunks::<16>();
                if !tail.is_empty() {
                    return Err(HyperlightError::Error("malformed execution timers".into()));
                }
                let entries = registry.entries.lock().unwrap();
                let now = Instant::now();
                for record in records {
                    let id = u64::from_le_bytes(record[..8].try_into().unwrap());
                    let ns = u64::from_le_bytes(record[8..].try_into().unwrap());
                    let entry = entries.get(&id).ok_or_else(|| {
                        HyperlightError::Error(format!("unknown timer execution {id}"))
                    })?;
                    let at = now
                        .checked_add(Duration::from_nanos(ns))
                        .ok_or_else(|| HyperlightError::Error("timer deadline overflow".into()))?;
                    let mut deadline = entry.deadline.lock().unwrap();
                    *deadline = Some(deadline.map_or(at, |old| old.min(at)));
                }
                Ok(0)
            },
        )?;
        Ok(())
    }

    pub fn events(&self, bytes: &[u8]) -> hyperlight_host::Result<()> {
        let (records, tail) = bytes.as_chunks::<24>();
        if !tail.is_empty() {
            return Err(HyperlightError::Error("malformed execution events".into()));
        }
        let entries = self.entries.lock().unwrap();
        for record in records {
            let id = u64::from_le_bytes(record[..8].try_into().unwrap());
            let pid = i32::from_le_bytes(record[8..12].try_into().unwrap());
            let error = i32::from_le_bytes(record[16..20].try_into().unwrap());
            let flags = u32::from_le_bytes(record[20..].try_into().unwrap());
            if !entries.contains_key(&id)
                || flags & !127 != 0
                || (flags & 1 != 0 && pid <= 0)
                || (flags & 64 != 0 && error <= 0)
            {
                return Err(HyperlightError::Error("invalid execution event".into()));
            }
        }
        drop(entries);
        for record in records {
            let id = u64::from_le_bytes(record[..8].try_into().unwrap());
            let entry = self
                .get(id)
                .ok_or_else(|| HyperlightError::Error(format!("unknown execution {id}")))?;
            let flags = u32::from_le_bytes(record[20..].try_into().unwrap());
            if flags & 1 != 0 {
                entry.event(0, i32::from_le_bytes(record[8..12].try_into().unwrap()))?;
            }
            if flags & 2 != 0 {
                entry.event(1, i32::from_le_bytes(record[12..16].try_into().unwrap()))?;
            }
            if flags & 64 != 0 {
                entry.event(0, -i32::from_le_bytes(record[16..20].try_into().unwrap()))?;
            }
            for (stream, pipe) in entry.pipes.iter().enumerate() {
                if flags & (8 << stream) != 0 {
                    if stream == 0 {
                        pipe.close();
                    } else {
                        pipe.eof();
                    }
                }
            }
            if flags & 4 != 0 {
                entry.done.store(true, Ordering::Relaxed);
            }
            entry.wake.notify();
        }
        Ok(())
    }
}

const PIPE_CAPACITY: usize = 64 * 1024;
const IO_CHUNK: usize = 4096;

fn checkpoint_inputs(
    entries: &[Arc<Execution>],
    replies: &[(u64, &[u8])],
    mut send: impl FnMut(Vec<u8>, &[u64]) -> crate::Result<()>,
) -> crate::Result<()> {
    let mut next = 0;
    let mut reply_next = 0;
    while next < entries.len() || reply_next < replies.len() {
        // Envelope version and stdin length, followed by two versioned batches.
        let mut batch = vec![1, 0, 0, 0, 0, 0, 0, 0];
        batch.extend(async_calls::empty_batch());
        let mut committed = Vec::new();
        while let Some(entry) = entries.get(next) {
            let state = entry.pipes[0].state.lock().unwrap();
            if state.closed
                || state.mode == crate::command::Stdio::Null
                || (state.bytes.is_empty() && (!state.eof || state.guest_eof))
            {
                next += 1;
                continue;
            }
            if batch.len() + 24 + state.bytes.len() + 8 > async_calls::MAX_BATCH_BYTES {
                if committed.is_empty() {
                    return Err(hyperlight_host::new_error!(
                        "stdin record exceeds checkpoint capacity"
                    )
                    .into());
                }
                break;
            }
            let received = state
                .guest_received
                .checked_add(state.bytes.len() as u64)
                .ok_or_else(|| hyperlight_host::new_error!("stdin sequence exhausted"))?;
            batch.extend(entry.id.to_le_bytes());
            batch.extend(state.guest_received.to_le_bytes());
            batch.extend((state.bytes.len() as u32).to_le_bytes());
            batch.extend(u32::from(state.eof).to_le_bytes());
            batch.extend(state.bytes.iter());
            committed.push((entry, received, state.eof));
            next += 1;
        }
        batch[12..16].copy_from_slice(&(committed.len() as u32).to_le_bytes());
        let stdin_len = batch.len() - 8;
        batch[4..8].copy_from_slice(&(stdin_len as u32).to_le_bytes());
        let reply_offset = batch.len();
        batch.extend(async_calls::empty_batch());
        let mut ids = Vec::new();
        while let Some(&(id, record)) = replies.get(reply_next) {
            if batch.len() + record.len() > async_calls::MAX_BATCH_BYTES {
                if committed.is_empty() && ids.is_empty() {
                    return Err(
                        hyperlight_host::new_error!("reply exceeds checkpoint capacity").into(),
                    );
                }
                break;
            }
            batch.extend_from_slice(record);
            ids.push(id);
            reply_next += 1;
        }
        if committed.is_empty() && ids.is_empty() {
            break;
        }
        batch[reply_offset + 4..reply_offset + 8]
            .copy_from_slice(&(ids.len() as u32).to_le_bytes());
        send(batch, &ids)?;
        // Stream gates are held throughout; acknowledge only a successful batch.
        for (entry, received, eof) in committed {
            let mut state = entry.pipes[0].state.lock().unwrap();
            state.guest_bytes += state.bytes.len();
            state.bytes.clear();
            state.guest_received = received;
            state.guest_eof = eof;
        }
    }
    Ok(())
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PipeState {
    bytes: VecDeque<u8>,
    guest_bytes: usize,
    guest_received: u64,
    guest_eof: bool,
    eof: bool,
    closed: bool,
    mode: crate::command::Stdio,
    dropped: u64,
}

impl Default for PipeState {
    fn default() -> Self {
        Self {
            bytes: VecDeque::new(),
            guest_bytes: 0,
            guest_received: 0,
            guest_eof: false,
            eof: false,
            closed: false,
            mode: crate::command::Stdio::Piped,
            dropped: 0,
        }
    }
}

#[derive(Default)]
pub(crate) struct Pipe {
    state: Mutex<PipeState>,
    reader: futures::task::AtomicWaker,
    writer: futures::task::AtomicWaker,
    readiness: futures::task::AtomicWaker,
}

impl Pipe {
    fn consume_stdin(&self, received: u64, unread: u64) -> hyperlight_host::Result<Vec<u8>> {
        let mut state = self.state.lock().unwrap();
        if received != state.guest_received
            || unread > state.guest_bytes as u64
            || unread > PIPE_CAPACITY as u64
        {
            return Err(hyperlight_host::new_error!(
                "invalid stdin queue accounting"
            ));
        }
        let consumed = state.guest_bytes - unread as usize;
        state.guest_bytes = unread as usize;
        let reply = vec![
            0,
            u8::from((state.eof || state.closed) && state.bytes.is_empty()),
        ];
        drop(state);
        if consumed != 0 {
            self.writer.wake();
            self.readiness.wake();
        }
        Ok(reply)
    }

    fn new(mode: crate::command::Stdio) -> Self {
        Self {
            state: Mutex::new(PipeState {
                mode,
                ..PipeState::default()
            }),
            ..Self::default()
        }
    }

    pub(crate) fn dropped_bytes(&self) -> u64 {
        self.state.lock().unwrap().dropped
    }

    pub(crate) fn read(&self, cx: &mut Context<'_>, out: &mut [u8]) -> Poll<io::Result<usize>> {
        let result = self.read_buffer(cx, out);
        if matches!(result, Poll::Ready(Ok(count)) if count != 0) {
            self.writer.wake();
            self.readiness.wake();
        }
        result
    }

    pub(crate) fn read_buffer(
        &self,
        cx: &mut Context<'_>,
        out: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.reader.register(cx.waker());
        let mut state = self.state.lock().unwrap();
        if out.is_empty() {
            return Poll::Ready(Ok(0));
        }
        let count = out.len().min(state.bytes.len());
        if count == 0 && !state.eof && !state.closed {
            return Poll::Pending;
        }
        let (first, second) = state.bytes.as_slices();
        let n = count.min(first.len());
        out[..n].copy_from_slice(&first[..n]);
        out[n..count].copy_from_slice(&second[..count - n]);
        state.bytes.drain(..count);
        Poll::Ready(Ok(count))
    }

    #[cfg(test)]
    fn write(&self, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<io::Result<usize>> {
        self.write_min(cx, bytes, 1)
    }

    pub(crate) fn write_buffer(
        &self,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.write_min_buffer(cx, bytes, 1)
    }

    fn write_chunk(&self, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<io::Result<usize>> {
        self.write_min(cx, bytes, bytes.len())
    }

    fn write_min(
        &self,
        cx: &mut Context<'_>,
        bytes: &[u8],
        minimum: usize,
    ) -> Poll<io::Result<usize>> {
        let result = self.write_min_buffer(cx, bytes, minimum);
        if matches!(result, Poll::Ready(Ok(count)) if count != 0) {
            self.reader.wake();
            self.readiness.wake();
        }
        result
    }

    fn write_min_buffer(
        &self,
        cx: &mut Context<'_>,
        bytes: &[u8],
        minimum: usize,
    ) -> Poll<io::Result<usize>> {
        self.writer.register(cx.waker());
        let mut state = self.state.lock().unwrap();
        if state.closed || state.eof {
            return Poll::Ready(Err(io::ErrorKind::BrokenPipe.into()));
        }
        use crate::command::Stdio;
        match state.mode {
            Stdio::Head(capacity) => {
                let count = bytes.len().min(capacity - state.bytes.len());
                state.bytes.extend(&bytes[..count]);
                state.dropped = state.dropped.saturating_add((bytes.len() - count) as u64);
            }
            Stdio::Tail(capacity) => {
                let retained = bytes.len().min(capacity);
                let evicted = state.bytes.len().saturating_sub(capacity - retained);
                state.bytes.drain(..evicted);
                state.bytes.extend(&bytes[bytes.len() - retained..]);
                state.dropped = state.dropped.saturating_add(evicted as u64);
                state.dropped = state
                    .dropped
                    .saturating_add((bytes.len() - retained) as u64);
            }
            Stdio::Unbounded => state.bytes.extend(bytes),
            Stdio::Null | Stdio::Piped => {
                let count = bytes
                    .len()
                    .min(PIPE_CAPACITY - state.bytes.len() - state.guest_bytes);
                if count < minimum && !bytes.is_empty() {
                    return Poll::Pending;
                }
                state.bytes.extend(&bytes[..count]);
                return Poll::Ready(Ok(count));
            }
        }
        Poll::Ready(Ok(bytes.len()))
    }

    pub(crate) fn eof(&self) {
        self.set_eof();
        self.notify();
    }

    pub(crate) fn set_eof(&self) {
        self.state.lock().unwrap().eof = true;
    }

    pub(crate) fn notify(&self) {
        self.reader.wake();
        self.writer.wake();
        self.readiness.wake();
    }

    pub(crate) fn close(&self) {
        self.set_closed();
        self.notify();
    }

    pub(crate) fn set_closed(&self) {
        let mut state = self.state.lock().unwrap();
        state.closed = true;
        state.bytes.clear();
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub(crate) struct ProcessState {
    pub pid: Option<u32>,
    pub status: Option<i32>,
    pub error: Option<i32>,
}

impl Execution {
    fn stdio_error(errno: i32) -> Vec<u8> {
        let mut reply = vec![1];
        reply.extend_from_slice(&errno.to_le_bytes());
        reply
    }

    fn stdio_read(&self, count: u64) -> hyperlight_host::Result<Vec<u8>> {
        if count > IO_CHUNK as u64 {
            return Ok(Self::stdio_error(errno::EINVAL));
        }
        let mut reply = vec![0; count as usize + 1];
        let mut cx = Context::from_waker(futures::task::noop_waker_ref());
        match self.pipes[0].read(&mut cx, &mut reply[1..]) {
            Poll::Ready(Ok(count)) => {
                reply.truncate(count + 1);
                Ok(reply)
            }
            Poll::Ready(Err(_)) => Ok(Self::stdio_error(errno::EPIPE)),
            Poll::Pending => Ok(Self::stdio_error(errno::EAGAIN)),
        }
    }

    fn stdio_write(&self, stream: u64, bytes: &[u8]) -> hyperlight_host::Result<Vec<u8>> {
        if !(1..=2).contains(&stream) || bytes.len() > IO_CHUNK {
            return Ok(Self::stdio_error(errno::EINVAL));
        }
        if bytes.is_empty() {
            return Ok(vec![0]);
        }
        let mut cx = Context::from_waker(futures::task::noop_waker_ref());
        match self.pipes[stream as usize].write_chunk(&mut cx, bytes) {
            Poll::Ready(Ok(_)) => Ok(vec![0]),
            Poll::Ready(Err(_)) => Ok(Self::stdio_error(errno::EPIPE)),
            Poll::Pending => Ok(Self::stdio_error(errno::EAGAIN)),
        }
    }

    fn stdio_poll(&self, stream: u64) -> hyperlight_host::Result<Vec<u8>> {
        let Some(pipe) = self.pipes.get(stream as usize).cloned() else {
            return Ok(Self::stdio_error(errno::EINVAL));
        };
        let mut reply = Box::pin(poll_fn(move |cx| {
            pipe.readiness.register(cx.waker());
            let state = pipe.state.lock().unwrap();
            // Linux poll bits on the guest wire, independent of the host OS.
            let events: u32 = if stream == 0 {
                if state.eof || state.closed {
                    1 | 16
                } else if !state.bytes.is_empty() || state.guest_bytes != 0 {
                    1
                } else {
                    0
                }
            } else if state.closed || state.eof {
                8
            } else if matches!(
                state.mode,
                crate::command::Stdio::Head(_)
                    | crate::command::Stdio::Tail(_)
                    | crate::command::Stdio::Unbounded
            ) || PIPE_CAPACITY - state.bytes.len() >= IO_CHUNK
            {
                4
            } else {
                0
            };
            if events == 0 {
                Poll::Pending
            } else {
                let mut reply = vec![0];
                reply.extend_from_slice(&events.to_le_bytes());
                Poll::Ready(reply)
            }
        }));
        if let Some(reply) = reply.as_mut().now_or_never() {
            return Ok(reply);
        }
        match self.inbox.lock().unwrap().submit_reply(reply) {
            Ok(id) => {
                let mut reply = vec![2];
                reply.extend_from_slice(&id.to_le_bytes());
                Ok(reply)
            }
            Err(_) => Ok(Self::stdio_error(crate::errno::ENOBUFS)),
        }
    }

    pub fn shutdown(&self) {
        // Driver polls hold a temporary coordinator reference, so coordinator
        // destruction cannot race an operation on this engine.
        let mut calls = match self.calls.lock() {
            Ok(calls) => calls,
            Err(error) => {
                tracing::error!("command driver panicked; cancelling its host work on shutdown");
                error.into_inner()
            }
        };
        calls.clear();
        drop(calls);
        self.net.reset();
        if !self.done.load(Ordering::Relaxed) {
            self.process.lock().unwrap().status = Some(9);
        }
        self.done.store(true, Ordering::Relaxed);
        self.driving
            .lock()
            .unwrap()
            .parked
            .store(true, Ordering::Relaxed);
        self.pipes[0].close();
        self.pipes[1].eof();
        self.pipes[2].eof();
        self.wake.notify();
    }

    fn new(
        id: u64,
        inbox: &Arc<Mutex<async_calls::Inbox>>,
        net: Option<&Arc<hostnet::Net>>,
        streams: [bool; 3],
        command: CommandMetadata,
    ) -> Self {
        let inbox = Arc::new(Mutex::new(inbox.lock().unwrap().child()));
        let pipes = command.stdio.map(|mode| Arc::new(Pipe::new(mode)));
        Self {
            id,
            command,
            driving: Mutex::new(DrivingState {
                parked: Arc::new(AtomicBool::new(false)),
                generation: 1,
                streams,
                resume: None,
                checkpoint_ready: false,
            }),
            needs_restore: AtomicBool::new(false),
            kill_requested: AtomicBool::new(false),
            failure: Mutex::default(),
            done: Arc::new(AtomicBool::new(false)),
            process: Mutex::default(),
            pipes,
            calls: Mutex::new(async_calls::PendingCalls::new(inbox.clone())),
            inbox,
            net: Arc::new(net.map_or_else(hostnet::Net::disabled, |net| net.fork())),
            wake: Arc::default(),
            deadline: Mutex::new(None),
        }
    }

    pub fn check_driver(&self, generation: u64) -> Result<()> {
        let state = self.driving.lock().unwrap();
        if state.parked.load(Ordering::Relaxed) || state.generation != generation {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "command handle was invalidated by parking",
            )));
        }
        Ok(())
    }

    pub fn park(&self, calls: &mut async_calls::PendingCalls) {
        if !self.driving.lock().unwrap().parked.load(Ordering::Relaxed) {
            return;
        }
        let batch = calls.suspend();
        let mut state = self.driving.lock().unwrap();
        if state.resume.is_none() {
            state.resume = Some(batch);
        }
        drop(state);
        self.net.reset();
        self.needs_restore.store(true, Ordering::Relaxed);
    }

    pub(crate) fn check(&self) -> Result<()> {
        if let Some(message) = self.failure.lock().unwrap().as_ref() {
            return Err(Error::Wait(io::Error::other(message.clone())));
        }
        if let Some(errno) = self.process.lock().unwrap().error {
            let kind = match errno {
                2 => io::ErrorKind::NotFound,
                13 => io::ErrorKind::PermissionDenied,
                8 | 22 => io::ErrorKind::InvalidInput,
                _ => io::ErrorKind::Other,
            };
            return Err(Error::Wait(io::Error::new(
                kind,
                format!("guest command failed with errno {errno}"),
            )));
        }
        Ok(())
    }

    pub(crate) fn event(&self, kind: i32, value: i32) -> hyperlight_host::Result<()> {
        if kind == 3 && value == 0 {
            self.pipes[0].close();
            return Ok(());
        }
        let mut state = self.process.lock().unwrap();
        match kind {
            0 if value > 0 && state.pid.is_none() => state.pid = Some(value as u32),
            0 if value < 0 => state.error = Some(-value),
            1 if state.status.is_none() => state.status = Some(value),
            2 if value > 0 => state.error = Some(value),
            _ => {
                return Err(hyperlight_host::HyperlightError::Error(
                    "invalid command event".into(),
                ));
            }
        }
        drop(state);
        if kind == 1 {
            self.pipes[0].close();
        }
        self.wake.notify();
        Ok(())
    }

    pub(crate) fn io(
        self: &Arc<Self>,
        stream: u64,
        bytes: Vec<u8>,
    ) -> hyperlight_host::Result<Vec<u8>> {
        let Some(pipe) = self.pipes.get(stream as usize).cloned() else {
            return Err(hyperlight_host::HyperlightError::Error(
                "invalid command stream".into(),
            ));
        };
        if bytes.len() > IO_CHUNK || (stream == 0 && !bytes.is_empty()) {
            return Err(hyperlight_host::HyperlightError::Error(
                "invalid command I/O size".into(),
            ));
        }
        if stream != 0 && bytes.is_empty() {
            pipe.eof();
            return Ok(vec![0]);
        }
        let mut reply = Box::pin(async move {
            let result = if stream == 0 {
                let mut reply = vec![0; IO_CHUNK + 1];
                match poll_fn(|cx| pipe.read(cx, &mut reply[1..])).await {
                    Ok(count) => {
                        reply.truncate(count + 1);
                        return reply;
                    }
                    Err(error) => Err(error),
                }
            } else {
                poll_fn(|cx| pipe.write_chunk(cx, &bytes)).await.map(|_| ())
            };
            match result {
                Ok(()) => vec![0],
                Err(error) => {
                    let mut reply = vec![1];
                    let _ = error;
                    reply.extend_from_slice(&32i32.to_le_bytes()); // Guest Linux EPIPE.
                    reply
                }
            }
        });
        // Only buffer I/O is polled here, never embedder host futures.
        if let Some(reply) = reply.as_mut().now_or_never() {
            return Ok(reply);
        }
        let id = self.inbox.lock().unwrap().submit_reply(reply);
        let id = match id {
            Ok(id) => id,
            Err(error) => {
                tracing::warn!(%error, "cannot queue command I/O");
                let mut reply = vec![1];
                reply.extend_from_slice(&crate::errno::ENOBUFS.to_le_bytes());
                return Ok(reply);
            }
        };
        let mut reply = vec![2];
        reply.extend_from_slice(&id.to_le_bytes());
        Ok(reply)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checkpoint_stdin(
        entries: &[Arc<Execution>],
        mut send: impl FnMut(Vec<u8>) -> crate::Result<()>,
    ) -> crate::Result<()> {
        checkpoint_inputs(entries, &[], |batch, ids| {
            assert!(ids.is_empty());
            assert!(batch.len() <= async_calls::MAX_BATCH_BYTES);
            let len = u32::from_le_bytes(batch[4..8].try_into().unwrap()) as usize;
            send(batch[8..8 + len].to_vec())
        })
    }

    #[test]
    fn checkpoints_combine_input_and_replies_and_bound_large_reply_batches() {
        let entry = execution();
        entry.pipes[0].state.lock().unwrap().bytes.extend(b"stdin");
        let records: Vec<_> = (1..=20u64)
            .map(|id| {
                [
                    id.to_le_bytes().as_slice(),
                    &65536u32.to_le_bytes(),
                    &vec![0; 65536],
                ]
                .concat()
            })
            .collect();
        let replies: Vec<_> = records
            .iter()
            .enumerate()
            .map(|(i, record)| (i as u64 + 1, record.as_slice()))
            .collect();
        let mut calls = 0;
        let mut received = Vec::new();
        checkpoint_inputs(std::slice::from_ref(&entry), &replies[..1], |batch, ids| {
            calls += 1;
            assert_eq!(u32::from_le_bytes(batch[12..16].try_into().unwrap()), 1);
            assert_eq!(ids, &[1]);
            Ok(())
        })
        .unwrap();
        assert_eq!(calls, 1);
        calls = 0;
        checkpoint_inputs(&[entry], &replies[1..], |batch, ids| {
            calls += 1;
            assert!(batch.len() <= async_calls::MAX_BATCH_BYTES);
            let offset = 8 + u32::from_le_bytes(batch[4..8].try_into().unwrap()) as usize;
            let records: Vec<_> = async_calls::batch_records(&batch[offset..]).collect();
            assert_eq!(records.len(), ids.len());
            for ((id, record), expected_id) in records.iter().zip(ids) {
                assert_eq!(id, expected_id);
                assert_eq!(*record, replies[*id as usize - 1].1);
            }
            received.extend_from_slice(ids);
            Ok(())
        })
        .unwrap();
        assert_eq!(calls, 2);
        assert_eq!(received, (2..=20).collect::<Vec<_>>());
    }

    fn execution() -> Arc<Execution> {
        let inbox = Arc::new(Mutex::new(async_calls::Inbox::default()));
        inbox.lock().unwrap().negotiated = true;
        Arc::new(Execution::new(
            1,
            &inbox,
            None,
            [true; 3],
            CommandMetadata {
                name: None,
                program: "/test".into(),
                args: Vec::new(),
                cwd: None,
                env: BTreeMap::new(),
                stdio: [crate::command::Stdio::piped(); 3],
            },
        ))
    }

    #[test]
    fn stdin_batches_are_bounded_acknowledged_and_skip_unchanged_state() {
        let entries: Vec<_> = (1..=20)
            .map(|id| {
                let mut entry = execution();
                Arc::get_mut(&mut entry).unwrap().id = id;
                entry.pipes[0].state.lock().unwrap().bytes = vec![id as u8; PIPE_CAPACITY].into();
                entry
            })
            .collect();
        let mut lengths = Vec::new();
        let mut seen = Vec::new();
        checkpoint_stdin(&entries, |batch| {
            assert!(batch.len() <= crate::CALL_PAYLOAD_MAX);
            lengths.push(batch.len());
            let records = u32::from_le_bytes(batch[4..8].try_into().unwrap());
            let mut offset = 8;
            for _ in 0..records {
                let id = u64::from_le_bytes(batch[offset..offset + 8].try_into().unwrap());
                let len = u32::from_le_bytes(batch[offset + 16..offset + 20].try_into().unwrap())
                    as usize;
                assert_eq!(len, PIPE_CAPACITY);
                assert!(
                    batch[offset + 24..offset + 24 + len]
                        .iter()
                        .all(|&b| b == id as u8)
                );
                seen.push(id);
                offset += 24 + len;
            }
            assert_eq!(offset, batch.len());
            Ok(())
        })
        .unwrap();
        assert_eq!(lengths.len(), 2);
        assert_eq!(seen, (1..=20).collect::<Vec<_>>());
        checkpoint_stdin(&entries, |_| {
            panic!("unchanged input must not enter the guest")
        })
        .unwrap();
        entries[0].pipes[0].eof();
        assert!(
            checkpoint_stdin(&entries, |_| Err(
                hyperlight_host::new_error!("rejected").into()
            ))
            .is_err()
        );
        assert!(!entries[0].pipes[0].state.lock().unwrap().guest_eof);
        let mut calls = 0;
        checkpoint_stdin(&entries, |batch| {
            calls += 1;
            assert_eq!(batch.len(), 32);
            assert_eq!(&batch[28..32], &1u32.to_le_bytes());
            Ok(())
        })
        .unwrap();
        assert_eq!(calls, 1);
        checkpoint_stdin(&entries, |_| panic!("EOF already committed")).unwrap();
    }

    #[test]
    fn stdin_batch_retry_retains_only_unacknowledged_input() {
        let entries: Vec<_> = (1..=20)
            .map(|id| {
                let mut entry = execution();
                Arc::get_mut(&mut entry).unwrap().id = id;
                entry.pipes[0].state.lock().unwrap().bytes = vec![id as u8; PIPE_CAPACITY].into();
                entry
            })
            .collect();
        let mut calls = 0;
        let mut failed = Vec::new();
        assert!(
            checkpoint_stdin(&entries, |batch| {
                calls += 1;
                if calls == 2 {
                    failed = batch;
                    return Err(hyperlight_host::new_error!("second batch rejected").into());
                }
                Ok(())
            })
            .is_err()
        );
        assert_eq!(calls, 2);
        assert!(entries[0].pipes[0].state.lock().unwrap().bytes.is_empty());
        assert_eq!(
            entries.last().unwrap().pipes[0]
                .state
                .lock()
                .unwrap()
                .bytes
                .len(),
            PIPE_CAPACITY
        );
        checkpoint_stdin(&entries, |batch| {
            assert_eq!(batch, failed);
            calls += 1;
            Ok(())
        })
        .unwrap();
        assert_eq!(calls, 3);
        checkpoint_stdin(&entries, |_| panic!("all input was acknowledged")).unwrap();
    }

    #[test]
    fn stdin_checkpoint_shares_capacity_and_preserves_eof_order() {
        let entry = execution();
        let pipe = &entry.pipes[0];
        let mut cx = Context::from_waker(futures::task::noop_waker_ref());
        let input = vec![b'a'; PIPE_CAPACITY];
        assert!(matches!(
            pipe.write(&mut cx, &input),
            Poll::Ready(Ok(PIPE_CAPACITY))
        ));
        assert!(
            checkpoint_stdin(std::slice::from_ref(&entry), |_| {
                Err(hyperlight_host::new_error!("failed batch").into())
            })
            .is_err()
        );
        assert_eq!(pipe.state.lock().unwrap().bytes.len(), PIPE_CAPACITY);
        checkpoint_stdin(std::slice::from_ref(&entry), |_| Ok(())).unwrap();
        assert!(pipe.consume_stdin(PIPE_CAPACITY as u64 + 1, 0).is_err());
        assert!(
            pipe.consume_stdin(PIPE_CAPACITY as u64, PIPE_CAPACITY as u64 + 1)
                .is_err()
        );
        assert!(pipe.state.lock().unwrap().bytes.is_empty());
        assert!(pipe.write(&mut cx, b"z").is_pending());
        // Consuming one guest byte, not staging it, restores one write credit.
        pipe.consume_stdin(PIPE_CAPACITY as u64, (PIPE_CAPACITY - 1) as u64)
            .unwrap();
        assert!(matches!(pipe.write(&mut cx, b"z"), Poll::Ready(Ok(1))));
        pipe.eof();
        assert_eq!(pipe.consume_stdin(PIPE_CAPACITY as u64, 0).unwrap(), [0, 0]);
        checkpoint_stdin(std::slice::from_ref(&entry), |_| Ok(())).unwrap();
        assert_eq!(
            pipe.consume_stdin(PIPE_CAPACITY as u64 + 1, 1).unwrap(),
            [0, 1]
        );
        assert_eq!(
            pipe.consume_stdin(PIPE_CAPACITY as u64 + 1, 0).unwrap(),
            [0, 1]
        );
    }

    #[test]
    fn batched_lifecycle_preserves_early_eof_and_final_status() {
        let registry = Executions::default();
        let entry = execution();
        registry.entries.lock().unwrap().insert(1, entry.clone());
        entry.io(1, b"kept".to_vec()).unwrap();
        let record = |pid: i32, status: i32, flags: u32| {
            [
                1u64.to_le_bytes().as_slice(),
                pid.to_le_bytes().as_slice(),
                status.to_le_bytes().as_slice(),
                0i32.to_le_bytes().as_slice(),
                flags.to_le_bytes().as_slice(),
            ]
            .concat()
        };
        registry.events(&record(42, 0, 1 | 16)).unwrap();
        assert_eq!(entry.process.lock().unwrap().pid, Some(42));
        assert!(!entry.done.load(Ordering::Relaxed));
        let mut bytes = [0; 8];
        assert_eq!(
            poll_fn(|cx| entry.pipes[1].read(cx, &mut bytes))
                .now_or_never()
                .unwrap()
                .unwrap(),
            4
        );
        assert_eq!(&bytes[..4], b"kept");
        assert_eq!(
            poll_fn(|cx| entry.pipes[1].read(cx, &mut bytes))
                .now_or_never()
                .unwrap()
                .unwrap(),
            0
        );
        registry
            .events(&record(42, 17 << 8, 2 | 4 | 8 | 32))
            .unwrap();
        assert_eq!(entry.process.lock().unwrap().status, Some(17 << 8));
        assert!(entry.done.load(Ordering::Relaxed));
        registry.complete_entry();
        assert!(registry.list().is_empty());
    }

    #[test]
    fn malformed_lifecycle_batch_does_not_publish_earlier_records() {
        let registry = Executions::default();
        let entry = execution();
        registry.entries.lock().unwrap().insert(1, entry.clone());
        let mut record = [0u8; 24];
        record[..8].copy_from_slice(&1u64.to_le_bytes());
        record[8..12].copy_from_slice(&42i32.to_le_bytes());
        record[20..].copy_from_slice(&1u32.to_le_bytes());
        let mut batch = record.to_vec();
        record[..8].copy_from_slice(&2u64.to_le_bytes());
        batch.extend_from_slice(&record);
        assert!(registry.events(&batch).is_err());
        assert!(registry.events(&batch[..25]).is_err());
        assert!(entry.process.lock().unwrap().pid.is_none());
    }

    #[test]
    fn ready_command_io_does_not_queue_async_replies() {
        let execution = execution();
        assert_eq!(execution.io(1, b"output".to_vec()).unwrap(), [0]);
        assert_eq!(
            poll_fn(|cx| execution.pipes[0].write(cx, b"input"))
                .now_or_never()
                .unwrap()
                .unwrap(),
            5
        );
        assert_eq!(execution.io(0, Vec::new()).unwrap(), b"\0input");
        execution.pipes[0].eof();
        assert_eq!(execution.io(0, Vec::new()).unwrap(), [0]);
        execution.pipes[2].close();
        assert_eq!(execution.io(2, vec![1]).unwrap(), [1, 32, 0, 0, 0]);
        let inbox = execution.inbox.lock().unwrap();
        assert!(!inbox.is_pending());
        assert_eq!(inbox.next_id(), 0);
    }

    #[test]
    fn direct_stdio_short_reads_and_failed_writes_do_not_consume_extra_data() {
        let execution = execution();
        execution.pipes[0]
            .state
            .lock()
            .unwrap()
            .bytes
            .extend(b"abcdef");
        assert_eq!(execution.stdio_read(1).unwrap(), b"\0a");
        assert_eq!(execution.stdio_read(2).unwrap(), b"\0bc");
        assert_eq!(execution.stdio_read(4096).unwrap(), b"\0def");
        assert_eq!(execution.stdio_read(1).unwrap(), [1, 11, 0, 0, 0]);
        execution.pipes[0].eof();
        assert_eq!(execution.stdio_read(1).unwrap(), [0]);

        let pipe = &execution.pipes[1];
        pipe.state.lock().unwrap().bytes.resize(PIPE_CAPACITY, b'x');
        assert_eq!(execution.stdio_write(1, b"!").unwrap(), [1, 11, 0, 0, 0]);
        assert_eq!(execution.stdio_write(1, b"").unwrap(), [0]);
        assert!(!pipe.state.lock().unwrap().eof);
        assert!(!execution.inbox.lock().unwrap().is_pending());
        let mut bytes = vec![0; PIPE_CAPACITY];
        poll_fn(|cx| pipe.read(cx, &mut bytes))
            .now_or_never()
            .unwrap()
            .unwrap();
        assert!(bytes.iter().all(|&byte| byte == b'x'));
        pipe.close();
        assert_eq!(execution.stdio_write(1, b"!").unwrap(), [1, 32, 0, 0, 0]);
    }

    #[test]
    fn direct_stdio_readiness_waits_without_consuming_or_committing_io() {
        for stream in 0..3 {
            let execution = execution();
            let pipe = &execution.pipes[stream];
            if stream != 0 {
                pipe.state.lock().unwrap().bytes.resize(PIPE_CAPACITY, b'x');
            }
            assert_eq!(execution.stdio_poll(stream as u64).unwrap()[0], 2);
            let mut calls = execution.calls.lock().unwrap();
            assert!(poll_fn(|cx| calls.poll_batch(cx)).now_or_never().is_none());
            if stream == 0 {
                poll_fn(|cx| pipe.write(cx, b"input"))
                    .now_or_never()
                    .unwrap()
                    .unwrap();
            } else {
                let mut bytes = [0; IO_CHUNK];
                poll_fn(|cx| pipe.read(cx, &mut bytes))
                    .now_or_never()
                    .unwrap()
                    .unwrap();
            }
            assert_eq!(
                poll_fn(|cx| calls.poll_batch(cx))
                    .now_or_never()
                    .unwrap()
                    .unwrap(),
                async_calls::Wake::Calls
            );
            assert_eq!(
                &calls.take_batch()[20..],
                &[0, if stream == 0 { 1 } else { 4 }, 0, 0, 0]
            );
            assert_eq!(
                pipe.state.lock().unwrap().bytes.len(),
                if stream == 0 {
                    5
                } else {
                    PIPE_CAPACITY - IO_CHUNK
                }
            );
        }
    }

    #[test]
    fn backpressured_command_io_completes_once_after_space_or_close() {
        for close in [false, true] {
            let execution = execution();
            let pipe = &execution.pipes[1];
            poll_fn(|cx| pipe.write(cx, &vec![b'a'; PIPE_CAPACITY]))
                .now_or_never()
                .unwrap()
                .unwrap();
            assert_eq!(execution.io(1, b"next".to_vec()).unwrap()[0], 2);
            let mut calls = execution.calls.lock().unwrap();
            assert!(poll_fn(|cx| calls.poll_batch(cx)).now_or_never().is_none());
            if close {
                pipe.close();
            } else {
                let mut first = [0; 4];
                assert_eq!(
                    poll_fn(|cx| pipe.read(cx, &mut first))
                        .now_or_never()
                        .unwrap()
                        .unwrap(),
                    4
                );
                assert_eq!(&first, b"aaaa");
            }
            assert_eq!(
                poll_fn(|cx| calls.poll_batch(cx))
                    .now_or_never()
                    .unwrap()
                    .unwrap(),
                async_calls::Wake::Calls
            );
            let batch = calls.take_batch();
            assert_eq!(
                &batch[20..],
                if close {
                    &[1, 32, 0, 0, 0][..]
                } else {
                    &[0][..]
                }
            );
            calls.delivered();
            assert!(!execution.inbox.lock().unwrap().is_pending());
            assert!(poll_fn(|cx| calls.poll_batch(cx)).now_or_never().is_none());
            if !close {
                let mut bytes = vec![0; PIPE_CAPACITY];
                assert_eq!(
                    poll_fn(|cx| pipe.read(cx, &mut bytes))
                        .now_or_never()
                        .unwrap()
                        .unwrap(),
                    PIPE_CAPACITY
                );
                assert!(bytes[..PIPE_CAPACITY - 4].iter().all(|&b| b == b'a'));
                assert_eq!(&bytes[PIPE_CAPACITY - 4..], b"next");
            }
        }
    }

    #[test]
    fn bulk_read_preserves_wrapped_and_partially_drained_bytes() {
        let pipe = Pipe::default();
        let mut bytes = VecDeque::with_capacity(8);
        bytes.extend(0..8);
        bytes.drain(..3);
        bytes.extend(8..11);
        assert!(!bytes.as_slices().1.is_empty());
        pipe.state.lock().unwrap().bytes = bytes;
        pipe.eof();
        let mut first = [0; 4];
        assert_eq!(
            poll_fn(|cx| pipe.read(cx, &mut first))
                .now_or_never()
                .unwrap()
                .unwrap(),
            4
        );
        assert_eq!(first, [3, 4, 5, 6]);
        let mut rest = [0; 8];
        assert_eq!(
            poll_fn(|cx| pipe.read(cx, &mut rest))
                .now_or_never()
                .unwrap()
                .unwrap(),
            4
        );
        assert_eq!(&rest[..4], &[7, 8, 9, 10]);
        assert_eq!(
            poll_fn(|cx| pipe.read(cx, &mut rest))
                .now_or_never()
                .unwrap()
                .unwrap(),
            0
        );
    }

    #[test]
    fn lossy_pipes_preserve_head_or_tail_across_reads_and_oversized_writes() {
        use crate::command::Stdio;

        fn ready(result: Poll<io::Result<usize>>) -> io::Result<usize> {
            match result {
                Poll::Ready(value) => value,
                Poll::Pending => panic!("lossy pipe unexpectedly backpressured"),
            }
        }

        for (mode, first, middle, last) in [
            (Stdio::head(4), b"abcd", b"cdgh", b"cdgh"),
            (Stdio::tail(4), b"cdef", b"hijk", b"6789"),
        ] {
            let pipe = Pipe::new(mode);
            let mut cx = Context::from_waker(futures::task::noop_waker_ref());
            assert_eq!(ready(pipe.write_chunk(&mut cx, b"abcdef")).unwrap(), 6);
            assert_eq!(pipe.state.lock().unwrap().bytes, first.as_slice());
            assert_eq!(pipe.dropped_bytes(), 2);
            let mut read = [0; 2];
            assert_eq!(ready(pipe.read(&mut cx, &mut read)).unwrap(), 2);
            assert_eq!(read, first[..2]);
            assert_eq!(ready(pipe.write_chunk(&mut cx, b"ghijk")).unwrap(), 5);
            assert_eq!(pipe.state.lock().unwrap().bytes, middle.as_slice());
            assert_eq!(pipe.dropped_bytes(), 5);
            assert_eq!(ready(pipe.write_chunk(&mut cx, b"0123456789")).unwrap(), 10);
            assert_eq!(pipe.dropped_bytes(), 15);
            assert_eq!(ready(pipe.write_chunk(&mut cx, b"")).unwrap(), 0);
            assert_eq!(pipe.dropped_bytes(), 15);
            pipe.eof();
            let mut read = [0; 8];
            assert_eq!(ready(pipe.read(&mut cx, &mut read)).unwrap(), 4);
            assert_eq!(&read[..4], last);
            assert_eq!(ready(pipe.read(&mut cx, &mut read)).unwrap(), 0);
            assert_eq!(
                ready(pipe.write_chunk(&mut cx, b"x")).unwrap_err().kind(),
                io::ErrorKind::BrokenPipe
            );
        }
    }

    #[test]
    fn nonbackpressuring_output_modes_stay_writable_until_closed() {
        use crate::command::Stdio;

        for mode in [
            Stdio::head(0),
            Stdio::tail(0),
            Stdio::head(3),
            Stdio::tail(3),
            Stdio::unbounded(),
        ] {
            let mut execution = execution();
            Arc::get_mut(&mut execution).unwrap().pipes[1] = Arc::new(Pipe::new(mode));
            let bytes = vec![b'x'; IO_CHUNK];
            for _ in 0..32 {
                assert_eq!(execution.stdio_write(1, &bytes).unwrap(), [0]);
            }
            assert_eq!(execution.stdio_poll(1).unwrap(), [0, 4, 0, 0, 0]);
            let pipe = &execution.pipes[1];
            let retained = match mode {
                Stdio::Head(n) | Stdio::Tail(n) => n,
                _ => 32 * IO_CHUNK,
            };
            assert_eq!(pipe.state.lock().unwrap().bytes.len(), retained);
            assert_eq!(pipe.dropped_bytes(), (32 * IO_CHUNK - retained) as u64);
            // The older CommandIo transport must use the same policy.
            assert_eq!(execution.io(1, b"x".to_vec()).unwrap(), [0]);
            pipe.close();
            assert_eq!(execution.stdio_poll(1).unwrap(), [0, 8, 0, 0, 0]);
            assert_eq!(
                execution.stdio_write(1, b"x").unwrap(),
                Execution::stdio_error(errno::EPIPE)
            );
        }
    }

    #[test]
    fn pipe_capacity_backpressures_and_eof_preserves_queued_bytes() {
        let pipe = Pipe::default();
        let bytes = vec![b'x'; PIPE_CAPACITY + 1];
        assert_eq!(
            poll_fn(|cx| pipe.write(cx, &bytes))
                .now_or_never()
                .unwrap()
                .unwrap(),
            PIPE_CAPACITY
        );
        assert!(poll_fn(|cx| pipe.write(cx, b"y")).now_or_never().is_none());
        let mut out = vec![0; PIPE_CAPACITY];
        assert_eq!(
            poll_fn(|cx| pipe.read(cx, &mut out))
                .now_or_never()
                .unwrap()
                .unwrap(),
            PIPE_CAPACITY
        );
        assert_eq!(out, bytes[..PIPE_CAPACITY]);
        assert_eq!(
            poll_fn(|cx| pipe.write(cx, b"y"))
                .now_or_never()
                .unwrap()
                .unwrap(),
            1
        );
        pipe.eof();
        assert_eq!(
            poll_fn(|cx| pipe.read(cx, &mut out))
                .now_or_never()
                .unwrap()
                .unwrap(),
            1
        );
        assert_eq!(out[0], b'y');
        assert_eq!(
            poll_fn(|cx| pipe.read(cx, &mut out))
                .now_or_never()
                .unwrap()
                .unwrap(),
            0
        );
    }

    #[test]
    fn closing_a_reader_breaks_a_backpressured_writer() {
        let pipe = Pipe::default();
        let bytes = vec![0; PIPE_CAPACITY];
        poll_fn(|cx| pipe.write(cx, &bytes))
            .now_or_never()
            .unwrap()
            .unwrap();
        assert!(
            poll_fn(|cx| pipe.write(cx, b"next"))
                .now_or_never()
                .is_none()
        );
        pipe.close();
        assert_eq!(
            poll_fn(|cx| pipe.write(cx, b"next"))
                .now_or_never()
                .unwrap()
                .unwrap_err()
                .kind(),
            io::ErrorKind::BrokenPipe
        );
        assert!(pipe.state.lock().unwrap().bytes.is_empty());
    }
}
