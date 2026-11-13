// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::collections::{BTreeMap, VecDeque};
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use futures::{
    future::poll_fn,
    task::{ArcWake, waker_ref},
};
use hyperlight_host::{HyperlightError, func::Registerable};

use crate::{Error, Result, async_calls, hostnet};

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
    pub attachment: Mutex<Attachment>,
    pub needs_restore: AtomicBool,
    pub failure: Mutex<Option<String>>,
    pub done: AtomicBool,
    pub process: Mutex<ProcessState>,
    pub pipes: [Arc<Pipe>; 3],
    pub inbox: Arc<Mutex<async_calls::Inbox>>,
    pub calls: Mutex<async_calls::PendingCalls>,
    pub net: Arc<hostnet::Net>,
    pub wake: Arc<WakeSet>,
    pub deadline: Mutex<Option<Instant>>,
}

#[derive(Clone)]
pub(crate) struct CommandMetadata {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: BTreeMap<String, String>,
    pub stdio: [crate::command::Stdio; 3],
}

pub(crate) struct Attachment {
    pub attached: bool,
    pub generation: u64,
    pub streams: [bool; 3],
    pub resume: Option<Vec<u8>>,
}

#[derive(Clone)]
pub(crate) struct ExecutionSnapshot {
    next: u64,
    next_request: u64,
    entries: Vec<ExecutionImage>,
}

#[derive(Clone)]
struct ExecutionImage {
    id: u64,
    command: CommandMetadata,
    done: bool,
    failure: Option<String>,
    process: ProcessState,
    pipes: [PipeState; 3],
    streams: [bool; 3],
    resume: Vec<u8>,
}

#[derive(Default)]
pub(crate) struct Executions {
    pub supported: AtomicBool,
    current: AtomicU64,
    active: AtomicU64,
    next: Mutex<u64>,
    entries: Mutex<BTreeMap<u64, Arc<Execution>>>,
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

    pub fn get(&self, id: u64) -> crate::Result<Arc<Execution>> {
        self.entries
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                Error::Wait(io::Error::new(
                    io::ErrorKind::NotFound,
                    "unknown command execution",
                ))
            })
    }

    pub fn list(&self) -> Vec<Arc<Execution>> {
        self.entries.lock().unwrap().values().cloned().collect()
    }

    pub fn forget(&self, id: u64) -> crate::Result<()> {
        let mut entries = self.entries.lock().unwrap();
        let entry = entries.get(&id).ok_or_else(|| {
            Error::Wait(io::Error::new(
                io::ErrorKind::NotFound,
                "unknown command execution",
            ))
        })?;
        if entry.attachment.lock().unwrap().attached || !entry.done.load(Ordering::Relaxed) {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::InvalidInput,
                "only a detached, completed command can be forgotten",
            )));
        }
        entries.remove(&id);
        Ok(())
    }

    pub fn snapshot(
        &self,
        inbox: &Arc<Mutex<async_calls::Inbox>>,
    ) -> crate::Result<ExecutionSnapshot> {
        let entries: Vec<_> = self.entries.lock().unwrap().values().cloned().collect();
        if entries
            .iter()
            .any(|entry| entry.attachment.lock().unwrap().attached)
        {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::InvalidInput,
                "drop all child handles before snapshot or restore",
            )));
        }
        let mut images = Vec::with_capacity(entries.len());
        for entry in entries {
            let mut calls = match entry.calls.try_lock() {
                Ok(calls) => calls,
                Err(std::sync::TryLockError::WouldBlock) => {
                    return Err(Error::Wait(io::Error::new(
                        io::ErrorKind::WouldBlock,
                        "a command poll is still detaching",
                    )));
                }
                Err(std::sync::TryLockError::Poisoned(error)) => panic!("{error}"),
            };
            entry.park(&mut calls);
            let state = entry.attachment.lock().unwrap();
            images.push(ExecutionImage {
                id: entry.id,
                command: entry.command.clone(),
                done: entry.done.load(Ordering::Relaxed),
                failure: entry.failure.lock().unwrap().clone(),
                process: entry.process.lock().unwrap().clone(),
                pipes: std::array::from_fn(|i| entry.pipes[i].state.lock().unwrap().clone()),
                streams: state.streams,
                resume: state.resume.clone().unwrap(),
            });
        }
        Ok(ExecutionSnapshot {
            next: *self.next.lock().unwrap(),
            next_request: inbox.lock().unwrap().next_id(),
            entries: images,
        })
    }

    pub fn restore(
        &self,
        snapshot: &ExecutionSnapshot,
        inbox: &Arc<Mutex<async_calls::Inbox>>,
        net: Option<&Arc<hostnet::Net>>,
    ) {
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
            let state = entry.attachment.get_mut().unwrap();
            state.attached = false;
            state.resume = Some(image.resume.clone());
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
            "CommandDriverVersion",
            move || -> hyperlight_host::Result<u64> {
                registry.supported.store(true, Ordering::Relaxed);
                Ok(5)
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
}

const PIPE_CAPACITY: usize = 64 * 1024;
const IO_CHUNK: usize = 4096;

#[derive(Clone, Default)]
struct PipeState {
    bytes: VecDeque<u8>,
    eof: bool,
    closed: bool,
}

#[derive(Default)]
pub(crate) struct Pipe {
    state: Mutex<PipeState>,
    reader: futures::task::AtomicWaker,
    writer: futures::task::AtomicWaker,
}

impl Pipe {
    pub(crate) fn read(&self, cx: &mut Context<'_>, out: &mut [u8]) -> Poll<io::Result<usize>> {
        self.reader.register(cx.waker());
        let mut state = self.state.lock().unwrap();
        if out.is_empty() {
            return Poll::Ready(Ok(0));
        }
        let count = out.len().min(state.bytes.len());
        if count == 0 && !state.eof && !state.closed {
            return Poll::Pending;
        }
        for byte in &mut out[..count] {
            *byte = state.bytes.pop_front().unwrap();
        }
        drop(state);
        if count != 0 {
            self.writer.wake();
        }
        Poll::Ready(Ok(count))
    }

    pub(crate) fn write(&self, cx: &mut Context<'_>, bytes: &[u8]) -> Poll<io::Result<usize>> {
        self.write_min(cx, bytes, 1)
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
        self.writer.register(cx.waker());
        let mut state = self.state.lock().unwrap();
        if state.closed || state.eof {
            return Poll::Ready(Err(io::ErrorKind::BrokenPipe.into()));
        }
        let count = bytes.len().min(PIPE_CAPACITY - state.bytes.len());
        if count < minimum && !bytes.is_empty() {
            return Poll::Pending;
        }
        state.bytes.extend(&bytes[..count]);
        drop(state);
        if count != 0 {
            self.reader.wake();
        }
        Poll::Ready(Ok(count))
    }

    pub(crate) fn eof(&self) {
        self.state.lock().unwrap().eof = true;
        self.reader.wake();
        self.writer.wake();
    }

    pub(crate) fn close(&self) {
        let mut state = self.state.lock().unwrap();
        state.closed = true;
        state.bytes.clear();
        drop(state);
        self.reader.wake();
        self.writer.wake();
    }
}

#[derive(Clone, Default)]
pub(crate) struct ProcessState {
    pub pid: Option<u32>,
    pub status: Option<i32>,
    pub error: Option<i32>,
}

impl Execution {
    fn new(
        id: u64,
        inbox: &Arc<Mutex<async_calls::Inbox>>,
        net: Option<&Arc<hostnet::Net>>,
        streams: [bool; 3],
        command: CommandMetadata,
    ) -> Self {
        let inbox = Arc::new(Mutex::new(inbox.lock().unwrap().child()));
        Self {
            id,
            command,
            attachment: Mutex::new(Attachment {
                attached: true,
                generation: 1,
                streams,
                resume: None,
            }),
            needs_restore: AtomicBool::new(false),
            failure: Mutex::default(),
            done: AtomicBool::new(false),
            process: Mutex::default(),
            pipes: std::array::from_fn(|_| Arc::default()),
            calls: Mutex::new(async_calls::PendingCalls::new(inbox.clone())),
            inbox,
            net: Arc::new(net.map_or_else(hostnet::Net::disabled, |net| net.fork())),
            wake: Arc::default(),
            deadline: Mutex::new(None),
        }
    }

    pub fn check_attachment(&self, generation: u64) -> Result<()> {
        let state = self.attachment.lock().unwrap();
        if !state.attached || state.generation != generation {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "command handle is detached",
            )));
        }
        Ok(())
    }

    pub fn park(&self, calls: &mut async_calls::PendingCalls) {
        if self.attachment.lock().unwrap().attached {
            return;
        }
        let batch = calls.suspend();
        let mut state = self.attachment.lock().unwrap();
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
        let id = self.inbox.lock().unwrap().submit_reply(async move {
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
    use futures::FutureExt;

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
