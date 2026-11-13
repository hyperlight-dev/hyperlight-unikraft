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
/// Snapshots require all child handles to be dropped. The legacy interpreter
/// API is intentionally unavailable.
#[derive(Clone)]
pub struct CommandSandbox(Arc<Coordinator>);

/// Stable execution identity within a command sandbox, distinct from a guest PID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionId(u64);

/// An observation of a command, without acquiring a driving handle.
///
/// Metadata may become stale as commands are attached, detached or completed.
/// This entry does not keep the sandbox alive or prevent snapshots.
#[derive(Clone, Debug)]
pub struct ChildInfo {
    /// Stable execution identity within the originating sandbox.
    pub execution_id: ExecutionId,
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
    /// Whether there was no child handle when observed. An attached child is not
    /// necessarily being polled.
    pub detached: bool,
    /// Whether the root, descendants and command workers have finished cleanup.
    pub completed: bool,
    sandbox: Weak<Coordinator>,
}

impl ChildInfo {
    /// Reacquire this command from its originating sandbox without driving it.
    ///
    /// Rechecks the current state, not the observed `detached` field. Like
    /// [`CommandSandbox::restore_child`], fails if already attached or still
    /// detaching. Also fails if forgotten or the sandbox has been dropped.
    pub fn restore(&self) -> Result<Child> {
        let coordinator = self.sandbox.upgrade().ok_or_else(|| {
            Error::Wait(io::Error::new(
                io::ErrorKind::NotFound,
                "command sandbox has been dropped",
            ))
        })?;
        CommandSandbox(coordinator).restore_child(self.execution_id)
    }
}

/// A whole-VM snapshot plus parked command metadata, streams and retained replies.
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
    /// Capture every parked command and the shared guest VM without driving it.
    ///
    /// Drop every child first. Taken pipes are already invalidated by that drop.
    /// Returns `WouldBlock` through [`Error::Wait`] if a poll is still detaching.
    /// The original sandbox remains usable and its commands stay parked.
    pub fn snapshot(&self) -> Result<CommandSnapshot> {
        let _transition = self.0.transition()?;
        self.0.failure()?;
        // Quiesce outside the VM lock: dropping user futures may call host code.
        let inbox = self.0.try_sandbox()?.config.async_inbox.clone();
        let executions = self.0.executions.snapshot(&inbox)?;
        let guest = self.0.try_sandbox()?.snapshot()?;
        Ok(CommandSnapshot { guest, executions })
    }

    /// Replace the guest and command state with a snapshot. All commands restore
    /// parked; use their saved execution IDs to reacquire handles.
    ///
    /// All current children must be dropped first. Old pipe handles remain invalid.
    pub fn restore(&self, snapshot: &CommandSnapshot) -> Result<()> {
        let _transition = self.0.transition()?;
        let inbox = self.0.try_sandbox()?.config.async_inbox.clone();
        self.0.executions.snapshot(&inbox)?;
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
    /// Includes attached, detached and completed commands until forgotten.
    /// Each entry can reacquire its child through [`ChildInfo::restore`].
    /// Observations are not an atomic snapshot of concurrent lifecycle changes.
    pub fn children(&self) -> Vec<ChildInfo> {
        self.0
            .executions
            .list()
            .into_iter()
            .map(|execution| {
                let detached = !execution.attachment.lock().unwrap().attached;
                let process = execution.process.lock().unwrap();
                ChildInfo {
                    execution_id: ExecutionId(execution.id),
                    program: execution.command.program.clone(),
                    args: execution.command.args.clone(),
                    cwd: execution.command.cwd.clone(),
                    env: execution.command.env.clone(),
                    stdio: execution.command.stdio,
                    pid: process.pid,
                    detached,
                    completed: execution.done.load(std::sync::atomic::Ordering::Relaxed),
                    sandbox: Arc::downgrade(&self.0),
                }
            })
            .collect()
    }

    /// List detached executions, including completed commands not yet forgotten.
    /// Use [`children`](Self::children) for metadata and per-entry restoration.
    pub fn parked_children(&self) -> Vec<ExecutionId> {
        self.0
            .executions
            .list()
            .into_iter()
            .filter(|execution| !execution.attachment.lock().unwrap().attached)
            .map(|execution| ExecutionId(execution.id))
            .collect()
    }

    /// Reacquire a detached command. Resources are reconstructed when first driven.
    ///
    /// Previously taken pipe handles remain invalid; the returned child owns new
    /// handles to the preserved streams. Fails if a handle is already attached,
    /// or an in-flight poll has not yet finished detaching.
    pub fn restore_child(&self, id: ExecutionId) -> Result<Child> {
        let _transition = self.0.transition()?;
        self.0.failure()?;
        let execution = self.0.executions.get(id.0)?;
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
            let state = execution.attachment.lock().unwrap();
            if state.attached {
                return Err(Error::Wait(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "command already has a child handle",
                )));
            }
        }
        execution.park(&mut calls);
        let generation = {
            let mut state = execution.attachment.lock().unwrap();
            state.generation = state.generation.checked_add(1).ok_or_else(|| {
                Error::Wait(io::Error::other("command handle generations exhausted"))
            })?;
            state.attached = true;
            state.generation
        };
        drop(calls);
        Ok(Child::attach(self.0.clone(), execution, generation))
    }

    /// Release retained metadata and output for a detached, completed command.
    /// Restore and kill unfinished commands before forgetting them.
    pub fn forget_child(&self, id: ExecutionId) -> Result<()> {
        let _transition = self.0.transition()?;
        self.0.executions.forget(id.0)
    }

    /// Build a command using an absolute guest executable path.
    pub fn command(&self, program: impl Into<String>) -> Command {
        Command {
            sandbox: self.clone(),
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
pub struct Command {
    sandbox: CommandSandbox,
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
        let coordinator = &self.sandbox.0;
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
        let child = Child::attach(coordinator.clone(), execution, 1);
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

    fn transition(&self) -> Result<MutexGuard<'_, ()>> {
        match self.lifecycle.try_lock() {
            Ok(guard) => Ok(guard),
            Err(TryLockError::WouldBlock) => Err(Error::Wait(io::Error::new(
                io::ErrorKind::WouldBlock,
                "another command lifecycle transition is in progress",
            ))),
            Err(TryLockError::Poisoned(error)) => panic!("{error}"),
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
        if let Err(error) = execution.check_attachment(generation) {
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
        let mut state = execution.attachment.lock().unwrap();
        if !state.attached {
            drop(state);
            execution.park(&mut calls);
            state = execution.attachment.lock().unwrap();
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
        let resuming = execution.attachment.lock().unwrap().resume.is_some();
        let polled = if resuming {
            Poll::Ready(Ok(async_calls::Wake::Calls))
        } else {
            calls.set_timer(*execution.deadline.lock().unwrap());
            // Poll embedder futures without the VM lock.
            execution.wake.with_context(|cx| calls.poll_batch(cx))
        };
        if let Err(error) = execution.check_attachment(generation) {
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
                execution.attachment.lock().unwrap().resume.take().unwrap()
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
/// Dropping a wait future is safe. Dropping this handle parks the tree, cancels
/// pending host work, and releases host sockets and timers without a VM entry.
/// An in-flight pipe poll finishes the teardown when it returns. Taken pipes
/// become invalid. Restore using [`CommandSandbox::restore_child`].
/// Root exit automatically kills any remaining descendants.
/// Waiting does not drain piped output; drain concurrently or use
/// [`wait_with_output`](Self::wait_with_output).
pub struct Child {
    id: u64,
    generation: u64,
    coordinator: Arc<Coordinator>,
    execution: Arc<Execution>,
    /// Piped stdin, when requested. Drop it to send EOF.
    pub stdin: Option<ChildStdin>,
    /// Piped stdout, when requested.
    pub stdout: Option<ChildStdout>,
    /// Piped stderr, when requested.
    pub stderr: Option<ChildStderr>,
}

impl Child {
    fn attach(coordinator: Arc<Coordinator>, execution: Arc<Execution>, generation: u64) -> Self {
        let streams = execution.attachment.lock().unwrap().streams;
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

    /// Identity used to reacquire this command after dropping its handle.
    pub fn execution_id(&self) -> ExecutionId {
        ExecutionId(self.id)
    }

    /// Synchronously stop the root, all descendants, and their workers.
    ///
    /// Uses one VM entry, without polling host futures or draining output.
    /// Unforwarded output is discarded; bytes already in host pipe buffers
    /// remain readable, followed by EOF. Taken stdin handles are closed too.
    /// After success, `wait()` is immediately ready with the root's status.
    /// Like other VM entries, this cannot preempt CPU-bound guest code.
    pub fn kill(&mut self) -> Result<()> {
        self.execution.check_attachment(self.generation)?;
        self.coordinator.failure()?;
        if !self
            .execution
            .done
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let mut sandbox = self.coordinator.sandbox.lock().unwrap();
            self.coordinator.executions.before_entry(self.id);
            let AppSandbox {
                config,
                sandbox: vm,
                ..
            } = &mut *sandbox;
            let result = config.enter(
                vm,
                "CommandKill",
                (self.id.to_string(), config.env_version()),
            );
            let result = self.coordinator.entered(result);
            drop(sandbox);
            self.coordinator.executions.wake_all();
            result?;
        }
        if !self
            .execution
            .done
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            return Err(Error::Wait(io::Error::other(
                "whole-tree termination did not complete in one VM entry",
            )));
        }
        self.execution.pipes[0].close();
        self.execution.pipes[1].eof();
        self.execution.pipes[2].eof();
        self.execution.check()?;
        if self.execution.process.lock().unwrap().status.is_none() {
            return Err(Error::Wait(io::Error::other(
                "execution ended without a root status",
            )));
        }
        Ok(())
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
        self.execution.check_attachment(self.generation)?;
        self.coordinator.failure()?;
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
            if let Some(status) = self.try_wait()? {
                return Poll::Ready(Ok(status));
            }
            if let Poll::Ready(Err(error)) =
                self.coordinator.poll(&self.execution, self.generation, cx)
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

impl Drop for Child {
    fn drop(&mut self) {
        {
            let mut state = self.execution.attachment.lock().unwrap();
            if state.generation != self.generation {
                return;
            }
            state.attached = false;
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
    }
}

struct ChildPipe {
    subscription: Subscription,
    coordinator: Arc<Coordinator>,
    execution: Arc<Execution>,
    generation: u64,
    stream: usize,
}

impl Drop for ChildPipe {
    fn drop(&mut self) {
        let mut state = self.execution.attachment.lock().unwrap();
        if !state.attached || state.generation != self.generation {
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
        action: impl FnOnce(&crate::execution::Pipe) -> Poll<io::Result<T>>,
    ) -> Poll<io::Result<T>> {
        let state = self.execution.attachment.lock().unwrap();
        if !state.attached || state.generation != self.generation {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "command pipe is detached",
            )));
        }
        if let Err(error) = self
            .coordinator
            .failure()
            .and_then(|_| self.execution.check())
        {
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
        if let Poll::Ready(result) = self.access(|pipe| pipe.read(cx, out)) {
            return Poll::Ready(result);
        }
        if let Poll::Ready(Err(error)) = self.coordinator.poll(&self.execution, self.generation, cx)
        {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        self.access(|pipe| pipe.read(cx, out))
    }
}

/// A command's piped standard input; implements `futures::io::AsyncWrite`.
pub struct ChildStdin(ChildPipe);
/// A command's piped standard output; implements `futures::io::AsyncRead`.
pub struct ChildStdout(ChildPipe);
/// A command's piped standard error; implements `futures::io::AsyncRead`.
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
        if let Poll::Ready(result) = self.0.access(|pipe| pipe.write(cx, bytes)) {
            return Poll::Ready(result);
        }
        if let Poll::Ready(Err(error)) =
            self.0
                .coordinator
                .poll(&self.0.execution, self.0.generation, cx)
        {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        self.0.access(|pipe| pipe.write(cx, bytes))
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.0.access(|_| Poll::Ready(Ok(())))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.0.access(|pipe| {
            pipe.eof();
            Poll::Ready(Ok(()))
        })
    }
}
