// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Independently driven executable process trees.
//!
//! Await handles on a Tokio runtime with its time and I/O drivers enabled.
//! VM entries are synchronous; executor timeouts cannot interrupt CPU-bound
//! guest code. The existing builder interrupt-handle hook remains available.

use std::collections::BTreeMap;
use std::io;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, TryLockError, Weak};
use std::task::{Context, Poll};

use futures::{
    future::poll_fn,
    io::{AsyncRead, AsyncReadExt, AsyncWrite},
};

use crate::{
    AppSandbox, Error, Result, Yield, async_calls,
    execution::{CommandMetadata, Execution, Subscription},
};

/// A sandbox dedicated to independently polled command trees.
///
/// There is no background executor. Poll a child's wait or its pipes to drive
/// that tree's host operations and guest threads. Other command trees do not run.
/// Snapshots preserve existing child handles; replacing the current guest from a
/// snapshot requires parking or dropping them first. The legacy interpreter API is unavailable.
///
/// This owner cannot be cloned. Command builders, children and pipes hold weak
/// references; keep this value alive while using them.
///
/// ```compile_fail
/// use hyperlight_unikraft::command::CommandSandbox;
/// fn duplicate(sandbox: CommandSandbox) {
///     let other = sandbox.clone();
/// }
/// ```
pub struct CommandSandbox(Arc<Coordinator>);

/// Stable execution identity within a command sandbox, distinct from a guest PID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionId(u64);

/// Command metadata and live lifecycle flags, without acquiring a driving handle.
///
/// Launch metadata and PID are observations; lifecycle methods read shared atomics.
/// This entry does not keep the sandbox or execution resources alive or prevent
/// snapshots. After in-place snapshot restoration, obtain new entries to observe
/// the replacement executions. Old entries retain their original executions' flags.
#[derive(Clone, Debug)]
pub struct ChildInfo {
    /// Stable execution identity within the originating sandbox.
    pub execution_id: ExecutionId,
    /// Optional sandbox-local name assigned at launch.
    pub name: Option<String>,
    /// Absolute guest executable path supplied at launch.
    pub program: String,
    /// Arguments supplied at launch, excluding the executable name (`argv[0]`).
    pub args: Vec<String>,
    /// Requested guest working directory; `None` means inherit the driver's cwd.
    /// This is not updated by subsequent guest directory changes.
    pub cwd: Option<String>,
    /// Full environment sent at launch, after inheritance, overrides and removals.
    /// Does not track subsequent guest changes. Values may contain secrets.
    pub env: BTreeMap<String, String>,
    /// Original stdin, stdout and stderr configurations, in that order.
    pub stdio: [Stdio; 3],
    /// Guest root PID, if the executable has started.
    pub pid: Option<u32>,
    parked: Arc<AtomicBool>,
    completed: Arc<AtomicBool>,
    sandbox: Weak<Coordinator>,
}

impl ChildInfo {
    fn new(coordinator: &Arc<Coordinator>, execution: &Execution) -> Self {
        let parked = execution.driving.lock().unwrap().parked.clone();
        let process = execution.process.lock().unwrap();
        Self {
            execution_id: ExecutionId(execution.id),
            name: execution.command.name.clone(),
            program: execution.command.program.clone(),
            args: execution.command.args.clone(),
            cwd: execution.command.cwd.clone(),
            env: execution.command.env.clone(),
            stdio: execution.command.stdio,
            pid: process.pid,
            parked,
            completed: execution.done.clone(),
            sandbox: Arc::downgrade(coordinator),
        }
    }

    /// Whether this execution is parked (has no child driving handle).
    ///
    /// A child with a driving handle is not necessarily being polled. Parking may still
    /// be finishing; this observation does not guarantee restoration will succeed.
    pub fn parked(&self) -> bool {
        self.parked.load(Ordering::Relaxed)
    }

    /// Whether the root, descendants and command workers have finished cleanup.
    pub fn completed(&self) -> bool {
        self.completed.load(Ordering::Relaxed)
    }

    /// Reacquire this command from its originating sandbox without driving it.
    ///
    /// Rechecks the current state rather than relying on a prior status read. Like
    /// [`CommandSandbox::restore_child`], fails if a driving handle exists or parking
    /// is still in progress. Also fails if forgotten or the sandbox has been dropped.
    pub fn restore(&self) -> Result<Child> {
        let coordinator = self.sandbox.upgrade().ok_or_else(|| {
            Error::Wait(io::Error::new(
                io::ErrorKind::NotFound,
                "command sandbox has been dropped",
            ))
        })?;
        coordinator.restore_child(self.execution_id)
    }
}

/// A whole-VM snapshot plus command metadata, streams and retained replies.
///
/// Contains no live host futures, sockets, timers or pipe handles. Restore in
/// place with [`CommandSandbox::restore`] or boot a separate sandbox with
/// [`crate::SandboxBuilder::from_command_snapshot`]. Host functions, mounts and
/// networking policy must be supplied again when booting a separate sandbox.
#[derive(Clone)]
pub struct CommandSnapshot {
    pub(crate) guest: Arc<crate::Snapshot>,
    pub(crate) executions: crate::execution::ExecutionSnapshot,
}

struct Coordinator {
    lifecycle: Mutex<()>,
    sandbox: Mutex<AppSandbox>,
    executions: Arc<crate::execution::Executions>,
    failure: Mutex<Option<String>>,
}

impl Drop for Coordinator {
    fn drop(&mut self) {
        for execution in self.executions.list() {
            execution.shutdown();
        }
    }
}

struct CleanupGuard<'a> {
    coordinator: &'a Coordinator,
    lifecycle: Option<MutexGuard<'a, ()>>,
}

impl Drop for CleanupGuard<'_> {
    fn drop(&mut self) {
        drop(self.lifecycle.take());
        self.coordinator.finish_pending_kills();
    }
}

impl AppSandbox {
    /// Consume an idle compiled-runtime sandbox, enabling concurrent commands.
    pub fn into_commands(self) -> Result<CommandSandbox> {
        self.check_ready()?;
        self.ensure_running()?;
        if self.config.call_in_flight() || self.async_calls.is_pending() || self.config.can_wake() {
            return Err(Error::CallInFlight);
        }
        if !self
            .config
            .executions
            .supported
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::Unsupported,
                "this rootfs does not support command protocol version 5",
            )));
        }
        Ok(CommandSandbox(Arc::new(Coordinator {
            lifecycle: Mutex::new(()),
            executions: self.config.executions.clone(),
            sandbox: Mutex::new(self),
            failure: Mutex::new(None),
        })))
    }
}

impl CommandSandbox {
    /// Capture every command and the shared guest VM without driving it.
    ///
    /// Children may be parked or have driving handles. Original handles, pipes,
    /// sockets and pending host calls are unchanged. Restored copies start parked;
    /// unfinished calls there are interrupted when driven. Completed data replies
    /// are retained, but replies tied to reset sockets are not.
    /// Returns `WouldBlock` through [`Error::Wait`] if a VM entry, child poll or
    /// handle/pipe operation prevents a consistent capture. Retry when it finishes.
    pub fn snapshot(&self) -> Result<CommandSnapshot> {
        let _transition = self.0.transition()?;
        self.0.failure()?;
        let result = {
            let mut sandbox = self.0.try_sandbox()?;
            let inbox = sandbox.config.async_inbox.clone();
            self.0.executions.snapshot(&inbox, || sandbox.snapshot())
        };
        // Polls that encountered our locks must get another chance to run.
        for execution in self.0.executions.list() {
            execution.wake.notify();
        }
        let (executions, guest) = result?;
        Ok(CommandSnapshot { guest, executions })
    }

    /// Replace the guest and command state with a snapshot. All commands restore
    /// parked; use their saved execution IDs to reacquire handles.
    ///
    /// All current children must be parked or dropped first. Old pipe handles remain invalid.
    pub fn restore(&self, snapshot: &CommandSnapshot) -> Result<()> {
        let _transition = self.0.transition()?;
        let inbox = self.0.try_sandbox()?.config.async_inbox.clone();
        self.0.executions.prepare_restore()?;
        let mut sandbox = self.0.try_sandbox()?;
        self.0
            .executions
            .restore(&snapshot.executions, &inbox, sandbox.config.net.as_ref());
        let result = sandbox.restore(snapshot.guest.clone());
        self.0.executions.negotiate(&inbox);
        *self.0.failure.lock().unwrap() = result.as_ref().err().map(ToString::to_string);
        result
    }

    /// Observe all retained commands in execution-ID order without driving them.
    ///
    /// Includes commands with driving handles, parked commands and completed commands
    /// until forgotten.
    /// Each entry can reacquire its child through [`ChildInfo::restore`].
    /// Observations are not an atomic snapshot of concurrent lifecycle changes.
    pub fn children(&self) -> Vec<ChildInfo> {
        self.0
            .executions
            .list()
            .into_iter()
            .map(|execution| ChildInfo::new(&self.0, &execution))
            .collect()
    }

    /// List parked executions, including completed commands not yet forgotten.
    /// Use [`children`](Self::children) for metadata and per-entry restoration.
    pub fn parked_children(&self) -> Vec<ExecutionId> {
        self.0
            .executions
            .list()
            .into_iter()
            .filter(|execution| {
                execution
                    .driving
                    .lock()
                    .unwrap()
                    .parked
                    .load(Ordering::Relaxed)
            })
            .map(|execution| ExecutionId(execution.id))
            .collect()
    }

    /// Reacquire a parked command. Resources are reconstructed when first driven.
    ///
    /// Previously taken pipe handles remain invalid; the returned child owns new
    /// handles to the preserved streams. Fails if a driving handle already exists,
    /// or an in-flight poll has not yet finished parking.
    pub fn restore_child(&self, id: ExecutionId) -> Result<Child> {
        self.0.restore_child(id)
    }

    /// Observe a retained command by its exact, case-sensitive launch name.
    ///
    /// Includes commands with driving handles, parked commands and completed
    /// commands. Does not drive or acquire the child; use [`ChildInfo::restore`]
    /// to restore it. Returns `None` for an unknown name.
    pub fn get_child_by_name(&self, name: &str) -> Option<ChildInfo> {
        let execution = self.0.executions.get_by_name(name)?;
        Some(ChildInfo::new(&self.0, &execution))
    }

    /// Release retained metadata and output for a parked, completed command.
    /// Restore and kill unfinished commands before forgetting them.
    pub fn forget_child(&self, id: ExecutionId) -> Result<()> {
        let _transition = self.0.transition()?;
        self.0.executions.forget(id.0)
    }

    /// Build a command using an absolute guest executable path.
    pub fn command(&self, program: impl Into<String>) -> Command {
        Command {
            sandbox: Arc::downgrade(&self.0),
            name: None,
            program: program.into(),
            args: Vec::new(),
            env: BTreeMap::new(),
            clear_env: false,
            cwd: None,
            stdio: [Stdio::Null; 3],
        }
    }
}

/// Configuration for an executable, not an interpreter request or shell string.
/// Holds a weak sandbox reference; spawning fails if the sandbox has been dropped.
pub struct Command {
    sandbox: Weak<Coordinator>,
    name: Option<String>,
    program: String,
    args: Vec<String>,
    env: BTreeMap<String, Option<String>>,
    clear_env: bool,
    cwd: Option<String>,
    stdio: [Stdio; 3],
}

/// A command's standard stream configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stdio {
    /// Connect to `/dev/null` (the default).
    Null,
    /// Expose a bounded, independently pollable host pipe.
    Piped,
}

impl Stdio {
    /// Connect to `/dev/null`.
    pub fn null() -> Self {
        Self::Null
    }
    /// Create a host pipe.
    pub fn piped() -> Self {
        Self::Piped
    }
}

impl Command {
    /// Assign a sandbox-local name, retained across parking and snapshots.
    ///
    /// Names are case-sensitive and must be nonempty and contain no NUL.
    /// Spawn rejects duplicates with `AlreadyExists`, including retained completed
    /// commands. Forget the old command before reusing its name.
    /// This host-side label does not change the guest executable or arguments.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Append one argument without shell parsing.
    pub fn arg(&mut self, value: impl Into<String>) -> &mut Self {
        self.args.push(value.into());
        self
    }
    /// Append arguments without shell parsing.
    pub fn args(&mut self, values: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.args.extend(values.into_iter().map(Into::into));
        self
    }
    /// Set an environment variable for this command only.
    pub fn env(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.env.insert(key.into(), Some(value.into()));
        self
    }
    /// Remove an inherited environment variable.
    pub fn env_remove(&mut self, key: impl Into<String>) -> &mut Self {
        self.env.insert(key.into(), None);
        self
    }
    /// Do not inherit the sandbox environment.
    pub fn env_clear(&mut self) -> &mut Self {
        self.clear_env = true;
        self
    }
    /// Set the guest working directory.
    pub fn current_dir(&mut self, path: impl Into<String>) -> &mut Self {
        self.cwd = Some(path.into());
        self
    }
    /// Configure stdin.
    pub fn stdin(&mut self, stdio: Stdio) -> &mut Self {
        self.stdio[0] = stdio;
        self
    }
    /// Configure stdout.
    pub fn stdout(&mut self, stdio: Stdio) -> &mut Self {
        self.stdio[1] = stdio;
        self
    }
    /// Configure stderr.
    pub fn stderr(&mut self, stdio: Stdio) -> &mut Self {
        self.stdio[2] = stdio;
        self
    }

    /// Launch without polling any command's host futures.
    ///
    /// Requires an absolute executable path; there is no PATH search. Accepted
    /// commands remain owned by the sandbox if their handles are dropped.
    pub fn spawn(&mut self) -> Result<Child> {
        let coordinator = self.sandbox.upgrade().ok_or_else(|| {
            Error::Wait(io::Error::new(
                io::ErrorKind::NotFound,
                "command sandbox has been dropped",
            ))
        })?;
        let _transition = coordinator.transition()?;
        coordinator.failure()?;
        let mut sandbox = coordinator.sandbox.lock().unwrap();
        let mut env = BTreeMap::new();
        if !self.clear_env {
            for entry in sandbox
                .config
                .env_str
                .lock()
                .unwrap()
                .split('\0')
                .filter(|s| !s.is_empty())
            {
                if let Some((key, value)) = entry.split_once('=') {
                    env.insert(key.to_owned(), value.to_owned());
                }
            }
        }
        for (key, value) in &self.env {
            if key.is_empty() || key.contains(['=', '\0']) {
                return Err(Error::Wait(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid environment key",
                )));
            }
            match value {
                Some(value) => {
                    env.insert(key.clone(), value.clone());
                }
                None => {
                    env.remove(key);
                }
            }
        }
        if !self.program.starts_with('/') {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::InvalidInput,
                "an absolute guest executable path is required",
            )));
        }
        if self.cwd.as_ref().is_some_and(String::is_empty) {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::InvalidInput,
                "the guest working directory cannot be empty",
            )));
        }
        let mut fields = vec![
            "5".to_owned(),
            "0".to_owned(),
            self.args.len().to_string(),
            env.len().to_string(),
            self.stdio
                .iter()
                .enumerate()
                .fold(0u8, |bits, (i, mode)| {
                    bits | (u8::from(*mode == Stdio::Piped) << i)
                })
                .to_string(),
            self.cwd.clone().unwrap_or_default(),
            self.program.clone(),
        ];
        fields.extend(self.args.iter().cloned());
        fields.extend(env.iter().map(|(key, value)| format!("{key}={value}")));
        if fields.iter().any(|field| field.contains('\0')) {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::InvalidInput,
                "command fields cannot contain NUL",
            )));
        }
        let size = fields.iter().map(|s| s.len() + 1).sum::<usize>() + 20;
        if size > crate::CALL_PAYLOAD_MAX {
            return Err(Error::CallTooLarge { bytes: size });
        }
        let (id, execution) = coordinator.executions.create(
            &sandbox.config.async_inbox,
            sandbox.config.net.as_ref(),
            self.stdio.map(|mode| mode == Stdio::Piped),
            CommandMetadata {
                name: self.name.clone(),
                program: self.program.clone(),
                args: self.args.clone(),
                cwd: self.cwd.clone(),
                env,
                stdio: self.stdio,
            },
        )?;
        fields[1] = id.to_string();
        let request = fields.join("\0") + "\0";
        coordinator.executions.before_entry(id);
        let AppSandbox {
            config,
            sandbox: vm,
            ..
        } = &mut *sandbox;
        let result = config.enter(vm, "CommandSpawn", (request, config.env_version()));
        let result = coordinator.entered(result);
        drop(sandbox);
        coordinator.executions.wake_all();
        result?;
        let child = Child::new(coordinator.clone(), execution, 1);
        child.try_wait()?;
        Ok(child)
    }

    /// Capture stdout and stderr while waiting, closing stdin.
    pub async fn output(&mut self) -> Result<Output> {
        self.stdout(Stdio::Piped).stderr(Stdio::Piped);
        self.spawn()?.wait_with_output().await
    }

    /// Launch and wait for root exit and command cleanup.
    pub async fn status(&mut self) -> Result<ExitStatus> {
        self.spawn()?.wait().await
    }
}

impl Coordinator {
    fn restore_child(self: &Arc<Self>, id: ExecutionId) -> Result<Child> {
        let _transition = self.transition()?;
        self.failure()?;
        let execution = self.executions.get(id.0)?;
        if execution.kill_requested.load(Ordering::Relaxed) {
            return Err(Error::Wait(io::Error::new(
                io::ErrorKind::WouldBlock,
                "a dropped command is awaiting termination",
            )));
        }
        let mut calls = match execution.calls.try_lock() {
            Ok(calls) => calls,
            Err(TryLockError::WouldBlock) => {
                return Err(Error::Wait(io::Error::new(
                    io::ErrorKind::WouldBlock,
                    "command is still being driven",
                )));
            }
            Err(TryLockError::Poisoned(error)) => panic!("{error}"),
        };
        {
            let state = execution.driving.lock().unwrap();
            if !state.parked.load(Ordering::Relaxed) {
                return Err(Error::Wait(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "command already has a child handle",
                )));
            }
        }
        execution.park(&mut calls);
        let generation = {
            let mut state = execution.driving.lock().unwrap();
            state.generation = state.generation.checked_add(1).ok_or_else(|| {
                Error::Wait(io::Error::other("command handle generations exhausted"))
            })?;
            state.parked.store(false, Ordering::Relaxed);
            state.generation
        };
        drop(calls);
        Ok(Child::new(self.clone(), execution, generation))
    }

    fn try_sandbox(&self) -> Result<MutexGuard<'_, AppSandbox>> {
        match self.sandbox.try_lock() {
            Ok(guard) => Ok(guard),
            Err(TryLockError::WouldBlock) => Err(Error::Wait(io::Error::new(
                io::ErrorKind::WouldBlock,
                "a VM entry is still in progress",
            ))),
            Err(TryLockError::Poisoned(error)) => panic!("{error}"),
        }
    }

    fn transition(&self) -> Result<CleanupGuard<'_>> {
        self.finish_pending_kills();
        match self.lifecycle.try_lock() {
            Ok(guard) => Ok(CleanupGuard {
                coordinator: self,
                lifecycle: Some(guard),
            }),
            Err(TryLockError::WouldBlock) => Err(Error::Wait(io::Error::new(
                io::ErrorKind::WouldBlock,
                "another command lifecycle transition is in progress",
            ))),
            Err(TryLockError::Poisoned(error)) => panic!("{error}"),
        }
    }

    fn kill_execution(&self, execution: &Execution, sandbox: &mut AppSandbox) -> Result<()> {
        self.failure()?;
        if !execution.done.load(Ordering::Relaxed) {
            self.executions.before_entry(execution.id);
            let AppSandbox {
                config,
                sandbox: vm,
                ..
            } = sandbox;
            self.entered(config.enter(
                vm,
                "CommandKill",
                (execution.id.to_string(), config.env_version()),
            ))?;
        }
        if !execution.done.load(Ordering::Relaxed) {
            return Err(Error::Wait(io::Error::other(
                "whole-tree termination did not complete in one VM entry",
            )));
        }
        execution.pipes[0].close();
        execution.pipes[1].eof();
        execution.pipes[2].eof();
        execution.check()?;
        if execution.process.lock().unwrap().status.is_none() {
            return Err(Error::Wait(io::Error::other(
                "execution ended without a root status",
            )));
        }
        Ok(())
    }

    fn finish_pending_kills(&self) {
        loop {
            let pending: Vec<_> = self
                .executions
                .list()
                .into_iter()
                .filter(|execution| execution.kill_requested.load(Ordering::Relaxed))
                .collect();
            if pending.is_empty() {
                return;
            }
            let lifecycle = match self.lifecycle.try_lock() {
                Ok(guard) => guard,
                Err(TryLockError::WouldBlock) => return,
                Err(TryLockError::Poisoned(error)) => {
                    tracing::error!("cannot terminate dropped commands: lifecycle lock poisoned");
                    error.into_inner()
                }
            };
            for execution in pending {
                let mut sandbox = match self.sandbox.try_lock() {
                    Ok(sandbox) => sandbox,
                    Err(TryLockError::WouldBlock) => return,
                    Err(TryLockError::Poisoned(error)) => {
                        tracing::error!("cannot terminate dropped command: VM lock poisoned");
                        execution.kill_requested.store(false, Ordering::Relaxed);
                        *execution.failure.lock().unwrap() = Some(error.to_string());
                        continue;
                    }
                };
                if !execution.kill_requested.swap(false, Ordering::Relaxed) {
                    continue;
                }
                let result = self.kill_execution(&execution, &mut sandbox);
                drop(sandbox);
                if let Err(error) = result {
                    tracing::error!(execution_id = execution.id, %error, "failed to terminate dropped command");
                    *execution.failure.lock().unwrap() = Some(error.to_string());
                }
                self.executions.wake_all();
            }
            // Recheck after releasing the gate: a callback may have dropped another child.
            drop(lifecycle);
        }
    }

    fn entered(&self, result: Result<Yield>) -> Result<()> {
        let result = match result {
            Ok(Yield::Exited { status }) => Err(Error::Wait(io::Error::other(format!(
                "command supervisor exited with {status}"
            )))),
            Ok(Yield::CallFailed { status }) => Err(Error::Wait(io::Error::other(format!(
                "command supervisor rejected control request: {status}"
            )))),
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        };
        if let Err(error) = &result {
            *self.failure.lock().unwrap() = Some(error.to_string());
        }
        result
    }

    fn failure(&self) -> Result<()> {
        match self.failure.lock().unwrap().as_ref() {
            Some(message) => Err(Error::Wait(io::Error::other(message.clone()))),
            None => Ok(()),
        }
    }

    fn poll(
        &self,
        execution: &Arc<Execution>,
        generation: u64,
        cx: &mut Context<'_>,
    ) -> Poll<Result<()>> {
        let _cleanup = CleanupGuard {
            coordinator: self,
            lifecycle: None,
        };
        if let Err(error) = execution.check_driver(generation) {
            return Poll::Ready(Err(error));
        }
        if let Err(error) = self.failure() {
            return Poll::Ready(Err(error));
        }
        if let Err(error) = execution.check() {
            return Poll::Ready(Err(error));
        }
        let mut calls = match execution.calls.try_lock() {
            Ok(calls) => calls,
            Err(TryLockError::WouldBlock) => return Poll::Pending,
            Err(TryLockError::Poisoned(error)) => panic!("{error}"),
        };
        let result = self.poll_locked(execution, generation, cx, &mut calls);
        if execution.done.load(std::sync::atomic::Ordering::Relaxed)
            || self.failure.lock().unwrap().is_some()
        {
            calls.clear();
        }
        let mut state = execution.driving.lock().unwrap();
        if state.parked.load(Ordering::Relaxed) {
            drop(state);
            execution.park(&mut calls);
            state = execution.driving.lock().unwrap();
        }
        drop(calls);
        drop(state);
        if result.is_ready() {
            self.executions.wake_all();
        }
        result
    }

    fn poll_locked(
        &self,
        execution: &Arc<Execution>,
        generation: u64,
        cx: &mut Context<'_>,
        calls: &mut async_calls::PendingCalls,
    ) -> Poll<Result<()>> {
        let resuming = execution.driving.lock().unwrap().resume.is_some();
        let polled = if resuming {
            Poll::Ready(Ok(async_calls::Wake::Calls))
        } else {
            calls.set_timer(*execution.deadline.lock().unwrap());
            // Poll embedder futures without the VM lock.
            execution.wake.with_context(|cx| calls.poll_batch(cx))
        };
        if let Err(error) = execution.check_driver(generation) {
            return Poll::Ready(Err(error));
        }
        if execution.done.load(std::sync::atomic::Ordering::Relaxed) {
            calls.clear();
            return Poll::Pending;
        }
        let wake = match polled {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(error)) => {
                *execution.failure.lock().unwrap() = Some(error.to_string());
                calls.clear();
                execution.wake.notify();
                return Poll::Ready(Err(Error::Wait(error)));
            }
            Poll::Ready(Ok(wake)) => wake,
        };
        let mut sandbox = match self.sandbox.try_lock() {
            Ok(sandbox) => sandbox,
            Err(TryLockError::WouldBlock) => {
                // Prepared call batches stay in the engine until delivered.
                if wake == async_calls::Wake::Timer {
                    calls.set_timer(Some(std::time::Instant::now()));
                }
                return Poll::Pending;
            }
            Err(TryLockError::Poisoned(error)) => panic!("{error}"),
        };
        self.executions.before_entry(execution.id);
        let AppSandbox {
            config,
            sandbox: vm,
            ..
        } = &mut *sandbox;
        let result = if wake == async_calls::Wake::Calls {
            let batch = if resuming {
                execution.driving.lock().unwrap().resume.take().unwrap()
            } else {
                calls.take_batch()
            };
            config.enter(vm, "step_async", (batch,))
        } else {
            config.enter(vm, "step", ())
        };
        let result = self.entered(result);
        if result.is_ok() && wake == async_calls::Wake::Calls {
            calls.delivered();
        }
        drop(sandbox);
        cx.waker().wake_by_ref();
        Poll::Ready(result)
    }
}

/// The root process's exit status (not the lifetime of its descendants).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExitStatus(i32);
impl ExitStatus {
    /// Whether the process exited with code zero.
    pub fn success(self) -> bool {
        self.code() == Some(0)
    }
    /// Exit code, or `None` when terminated by a signal.
    pub fn code(self) -> Option<i32> {
        (self.0 & 0x7f == 0).then_some((self.0 >> 8) & 0xff)
    }
    /// Terminating guest signal, if any.
    pub fn signal(self) -> Option<i32> {
        (self.0 & 0x7f != 0).then_some(self.0 & 0x7f)
    }
}

/// Captured output and root exit status.
#[derive(Debug)]
pub struct Output {
    /// Root exit status.
    pub status: ExitStatus,
    /// Captured stdout, including descendants.
    pub stdout: Vec<u8>,
    /// Captured stderr, including descendants.
    pub stderr: Vec<u8>,
}

/// A root process and handles driving its entire guest process tree.
///
/// Dropping a borrowed wait future is safe. Dropping this handle terminates its
/// tree and invalidates taken pipes. Use [`park`](Self::park) to consume the handle
/// without killing the tree, then restore using [`CommandSandbox::restore_child`].
/// Drop normally terminates synchronously without polling host futures. If a VM
/// entry or lifecycle operation is in progress, termination is deferred until it
/// releases its locks. Drop failures are logged and retained as execution errors;
/// use [`kill`](Self::kill) to receive errors directly.
/// Root exit automatically kills any remaining descendants.
/// This handle and its pipes do not keep the sandbox alive. When its last owner
/// is dropped, unfinished commands report SIGKILL (9) without a guest VM entry;
/// completed commands retain their status. Buffered output can be drained to EOF.
/// A synchronous operation already in progress may finish before teardown.
/// Waiting does not drain piped output; drain concurrently or use
/// [`wait_with_output`](Self::wait_with_output).
pub struct Child {
    id: u64,
    generation: u64,
    coordinator: Weak<Coordinator>,
    execution: Arc<Execution>,
    /// Piped stdin, when requested. Drop it to send EOF.
    pub stdin: Option<ChildStdin>,
    /// Piped stdout, when requested.
    pub stdout: Option<ChildStdout>,
    /// Piped stderr, when requested.
    pub stderr: Option<ChildStderr>,
}

impl Child {
    fn new(coordinator: Arc<Coordinator>, execution: Arc<Execution>, generation: u64) -> Self {
        let coordinator = Arc::downgrade(&coordinator);
        let streams = execution.driving.lock().unwrap().streams;
        let pipe = |stream| ChildPipe {
            subscription: execution.wake.subscribe(),
            coordinator: coordinator.clone(),
            execution: execution.clone(),
            generation,
            stream,
        };
        Self {
            id: execution.id,
            generation,
            stdin: streams[0].then(|| ChildStdin(pipe(0))),
            stdout: streams[1].then(|| ChildStdout(pipe(1))),
            stderr: streams[2].then(|| ChildStderr(pipe(2))),
            coordinator,
            execution,
        }
    }

    /// Identity used to reacquire this command after parking its handle.
    pub fn execution_id(&self) -> ExecutionId {
        ExecutionId(self.id)
    }

    /// Optional sandbox-local name assigned at launch.
    pub fn name(&self) -> Option<&str> {
        self.execution.command.name.as_deref()
    }

    /// Consume this handle without killing the tree, returning its execution ID.
    ///
    /// Cancels pending host work and releases host sockets and timers without VM
    /// entry. Taken pipes become invalid; logical streams and buffered data remain
    /// available to a restored child. An in-flight poll finishes parking on return.
    pub fn park(self) -> ExecutionId {
        self.release(false);
        self.execution_id()
    }

    /// Synchronously stop the root, all descendants, and their workers.
    ///
    /// Uses one VM entry, without polling host futures or draining output.
    /// Unforwarded output is discarded; bytes already in host pipe buffers
    /// remain readable, followed by EOF. Taken stdin handles are closed too.
    /// After success, `wait()` is immediately ready with the root's status.
    /// Like other VM entries, this cannot preempt CPU-bound guest code.
    /// Returns `WouldBlock` if another lifecycle operation (such as snapshot
    /// capture) is in progress.
    pub fn kill(&mut self) -> Result<()> {
        let Some(coordinator) = self.coordinator.upgrade() else {
            return Ok(());
        };
        let _transition = coordinator.transition()?;
        self.execution.check_driver(self.generation)?;
        let result =
            coordinator.kill_execution(&self.execution, &mut coordinator.sandbox.lock().unwrap());
        coordinator.executions.wake_all();
        result
    }
    /// Guest root PID once the executable has started.
    pub fn id(&self) -> Option<u32> {
        self.execution.process.lock().unwrap().pid
    }
    /// Observe command completion without driving guest work.
    ///
    /// Returns the root's status only after descendant and worker cleanup,
    /// matching [`wait`](Self::wait).
    pub fn try_wait(&self) -> Result<Option<ExitStatus>> {
        let Some(coordinator) = self.coordinator.upgrade() else {
            return Ok(Some(self.terminated_status()));
        };
        self.execution.check_driver(self.generation)?;
        coordinator.failure()?;
        self.execution.check()?;
        if !self
            .execution
            .done
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            return Ok(None);
        }
        self.execution
            .process
            .lock()
            .unwrap()
            .status
            .map(|status| Some(ExitStatus(status)))
            .ok_or_else(|| Error::Wait(io::Error::other("execution ended without a root status")))
    }

    fn terminated_status(&self) -> ExitStatus {
        if self.execution.done.load(Ordering::Relaxed)
            && let Some(status) = self.execution.process.lock().unwrap().status
        {
            return ExitStatus(status);
        }
        ExitStatus(9)
    }

    /// Wait for root exit and command cleanup, returning the root's status.
    ///
    /// Root exit automatically kills remaining descendants, including reparented
    /// ones. Output already written is forwarded before cleanup completes.
    /// Cancellation is safe. Descendant exit codes are not aggregated.
    /// Piped output must be drained concurrently or via
    /// [`wait_with_output`](Self::wait_with_output).
    pub async fn wait(&mut self) -> Result<ExitStatus> {
        self.stdin.take();
        let subscription = self.execution.wake.subscribe();
        poll_fn(|cx| {
            subscription.register(cx.waker());
            let Some(coordinator) = self.coordinator.upgrade() else {
                return Poll::Ready(Ok(self.terminated_status()));
            };
            if let Some(status) = self.try_wait()? {
                return Poll::Ready(Ok(status));
            }
            if let Poll::Ready(Err(error)) = coordinator.poll(&self.execution, self.generation, cx)
            {
                return Poll::Ready(Err(error));
            }
            Poll::Pending
        })
        .await
    }
    /// Drain both pipes to EOF, wait for command cleanup, and collect the root status.
    ///
    /// Collected output is unbounded, like `std::process::Command::output`.
    pub async fn wait_with_output(mut self) -> Result<Output> {
        let stdout = self.stdout.take();
        let stderr = self.stderr.take();
        let capture = async |pipe: Option<ChildPipe>| -> Result<Vec<u8>> {
            let mut out = Vec::new();
            if let Some(mut pipe) = pipe {
                pipe.read_to_end(&mut out).await.map_err(Error::Wait)?;
            }
            Ok(out)
        };
        let (status, stdout, stderr) = futures::try_join!(
            self.wait(),
            capture(stdout.map(|p| p.0)),
            capture(stderr.map(|p| p.0)),
        )?;
        Ok(Output {
            status,
            stdout,
            stderr,
        })
    }
}

impl Child {
    fn release(&self, kill: bool) {
        let Some(coordinator) = self.coordinator.upgrade() else {
            return;
        };
        {
            let state = self.execution.driving.lock().unwrap();
            if state.generation != self.generation || state.parked.load(Ordering::Relaxed) {
                return;
            }
            if kill && !self.execution.done.load(Ordering::Relaxed) {
                self.execution.kill_requested.store(true, Ordering::Relaxed);
            }
            state.parked.store(true, Ordering::Relaxed);
        }
        match self.execution.calls.try_lock() {
            Ok(mut calls) => self.execution.park(&mut calls),
            Err(TryLockError::WouldBlock) => {}
            Err(TryLockError::Poisoned(error)) => {
                tracing::error!("command driver panicked; cancelling its host work");
                *self.execution.failure.lock().unwrap() = Some("command driver panicked".into());
                self.execution.park(&mut error.into_inner());
            }
        }
        self.execution.wake.notify();
        if kill {
            coordinator.finish_pending_kills();
        }
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        self.release(true);
    }
}

struct ChildPipe {
    subscription: Subscription,
    coordinator: Weak<Coordinator>,
    execution: Arc<Execution>,
    generation: u64,
    stream: usize,
}

impl Drop for ChildPipe {
    fn drop(&mut self) {
        let Some(_coordinator) = self.coordinator.upgrade() else {
            return;
        };
        let mut state = self.execution.driving.lock().unwrap();
        if state.parked.load(Ordering::Relaxed) || state.generation != self.generation {
            return;
        }
        state.streams[self.stream] = false;
        if self.stream == 0 {
            self.execution.pipes[0].eof();
        } else {
            self.execution.pipes[self.stream].close();
        }
    }
}

impl ChildPipe {
    fn access<T>(
        &self,
        coordinator: &Coordinator,
        action: impl FnOnce(&crate::execution::Pipe) -> Poll<io::Result<T>>,
    ) -> Poll<io::Result<T>> {
        let state = self.execution.driving.lock().unwrap();
        if state.parked.load(Ordering::Relaxed) || state.generation != self.generation {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "command pipe was invalidated by parking",
            )));
        }
        if let Err(error) = coordinator.failure().and_then(|_| self.execution.check()) {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        action(&self.execution.pipes[self.stream])
    }
}

impl AsyncRead for ChildPipe {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        out: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.subscription.register(cx.waker());
        let Some(coordinator) = self.coordinator.upgrade() else {
            let pipe = &self.execution.pipes[self.stream];
            pipe.eof();
            return pipe.read(cx, out);
        };
        if let Poll::Ready(result) = self.access(&coordinator, |pipe| pipe.read(cx, out)) {
            return Poll::Ready(result);
        }
        if let Poll::Ready(Err(error)) = coordinator.poll(&self.execution, self.generation, cx) {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        self.access(&coordinator, |pipe| pipe.read(cx, out))
    }
}

/// A command's piped standard input; implements `futures::io::AsyncWrite`.
/// Does not keep the sandbox alive. After sandbox destruction, writes and
/// flushes return `BrokenPipe`; closing is successful.
pub struct ChildStdin(ChildPipe);
/// A command's piped standard output; implements `futures::io::AsyncRead`.
/// Does not keep the sandbox alive. After sandbox destruction, buffered bytes
/// remain readable, followed by EOF.
pub struct ChildStdout(ChildPipe);
/// A command's piped standard error; implements `futures::io::AsyncRead`.
/// Has the same sandbox lifetime and EOF behavior as [`ChildStdout`].
pub struct ChildStderr(ChildPipe);

impl AsyncRead for ChildStdout {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        out: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, out)
    }
}
impl AsyncRead for ChildStderr {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        out: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_read(cx, out)
    }
}
impl AsyncWrite for ChildStdin {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.0.subscription.register(cx.waker());
        let Some(coordinator) = self.0.coordinator.upgrade() else {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "command sandbox has been dropped",
            )));
        };
        if let Poll::Ready(result) = self.0.access(&coordinator, |pipe| pipe.write(cx, bytes)) {
            return Poll::Ready(result);
        }
        if let Poll::Ready(Err(error)) = coordinator.poll(&self.0.execution, self.0.generation, cx)
        {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        self.0.access(&coordinator, |pipe| pipe.write(cx, bytes))
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        let Some(coordinator) = self.0.coordinator.upgrade() else {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "command sandbox has been dropped",
            )));
        };
        self.0.access(&coordinator, |_| Poll::Ready(Ok(())))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        let Some(coordinator) = self.0.coordinator.upgrade() else {
            return Poll::Ready(Ok(()));
        };
        self.0.access(&coordinator, |pipe| {
            pipe.eof();
            Poll::Ready(Ok(()))
        })
    }
}
