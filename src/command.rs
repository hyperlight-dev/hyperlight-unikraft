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
use std::sync::{Arc, Mutex, TryLockError};
use std::task::{Context, Poll};

use futures::{
    future::poll_fn,
    io::{AsyncRead, AsyncReadExt, AsyncWrite},
};

use crate::{
    AppSandbox, Error, Result, Yield, async_calls,
    execution::{Execution, Subscription},
};

/// A sandbox dedicated to independently polled command trees.
///
/// There is no background executor. Poll a child's wait or its pipes to drive
/// that tree's host operations. Guest CPU scheduling remains shared.
/// Snapshots and the legacy interpreter API are intentionally unavailable.
#[derive(Clone)]
pub struct CommandSandbox(Arc<Coordinator>);

struct Coordinator {
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
                "this rootfs does not support command protocol version 2",
            )));
        }
        Ok(CommandSandbox(Arc::new(Coordinator {
            executions: self.config.executions.clone(),
            sandbox: Mutex::new(self),
            failure: Mutex::new(None),
        })))
    }
}

impl CommandSandbox {
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
#[derive(Clone, Copy, PartialEq, Eq)]
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
            "2".to_owned(),
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
        fields.extend(env.into_iter().map(|(key, value)| format!("{key}={value}")));
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
        let (id, execution) = coordinator
            .executions
            .create(&sandbox.config.async_inbox, sandbox.config.net.as_ref())?;
        fields[1] = id.to_string();
        let request = fields.join("\0") + "\0";
        coordinator.executions.before_entry();
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
        let make_pipe = |stream: usize| ChildPipe {
            subscription: execution.wake.subscribe(),
            coordinator: coordinator.clone(),
            execution: execution.clone(),
            stream,
        };
        let child = Child {
            id,
            stdin: (self.stdio[0] == Stdio::Piped).then(|| ChildStdin(make_pipe(0))),
            stdout: (self.stdio[1] == Stdio::Piped).then(|| ChildStdout(make_pipe(1))),
            stderr: (self.stdio[2] == Stdio::Piped).then(|| ChildStderr(make_pipe(2))),
            execution,
            coordinator: coordinator.clone(),
        };
        child.try_wait()?;
        Ok(child)
    }

    /// Capture stdout and stderr while waiting, closing stdin.
    pub async fn output(&mut self) -> Result<Output> {
        self.stdout(Stdio::Piped).stderr(Stdio::Piped);
        self.spawn()?.wait_with_output().await
    }

    /// Launch and wait for the root process.
    pub async fn status(&mut self) -> Result<ExitStatus> {
        self.spawn()?.wait().await
    }
}

impl Coordinator {
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

    fn poll(&self, execution: &Arc<Execution>, cx: &mut Context<'_>) -> Poll<Result<()>> {
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
        calls.set_timer(*execution.deadline.lock().unwrap());
        // Poll embedder futures without the VM lock.
        let polled = execution.wake.with_context(|cx| calls.poll_batch(cx));
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
        self.executions.before_entry();
        let AppSandbox {
            config,
            sandbox: vm,
            ..
        } = &mut *sandbox;
        let result = if wake == async_calls::Wake::Calls {
            config.enter(vm, "step_async", (calls.take_batch(),))
        } else {
            config.enter(vm, "step", ())
        };
        let result = self.entered(result);
        if result.is_ok() && wake == async_calls::Wake::Calls {
            calls.delivered();
        }
        drop(sandbox);
        drop(calls);
        self.executions.wake_all();
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
/// Dropping a wait future is safe. Dropping this handle does not kill the tree.
/// Waiting does not drain piped output; drain concurrently or use
/// [`wait_with_output`](Self::wait_with_output).
pub struct Child {
    id: u64,
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
    /// Request SIGKILL for the root process, not its descendants.
    pub fn start_kill(&mut self) -> Result<()> {
        self.coordinator.failure()?;
        if self.execution.process.lock().unwrap().status.is_some() {
            return Ok(());
        }
        self.request_kill("CommandKill")
    }

    fn request_kill(&self, command: &str) -> Result<()> {
        let mut sandbox = self.coordinator.sandbox.lock().unwrap();
        self.coordinator.executions.before_entry();
        let AppSandbox {
            config,
            sandbox: vm,
            ..
        } = &mut *sandbox;
        let result = config.enter(vm, command, (self.id.to_string(), config.env_version()));
        let result = self.coordinator.entered(result);
        drop(sandbox);
        self.coordinator.executions.wake_all();
        result
    }

    /// Kill the root process and await its exit. Descendants remain independent.
    pub async fn kill(&mut self) -> Result<()> {
        self.start_kill()?;
        self.wait().await?;
        Ok(())
    }

    /// Kill the root and all descendants, including reparented descendants,
    /// then wait for the whole tree and its workers to finish.
    ///
    /// Closes stdin, including a taken stdin handle. Piped output is preserved
    /// and must be drained concurrently to avoid backpressure blocking cleanup.
    /// Cancellation leaves the kill request in effect; resume with `wait_all`.
    pub async fn kill_all(&mut self) -> Result<()> {
        self.coordinator.failure()?;
        if !self
            .execution
            .done
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.request_kill("CommandKillAll")?;
            self.execution.pipes[0].eof();
        }
        self.wait_all().await?;
        Ok(())
    }
    /// Guest root PID once the executable has started.
    pub fn id(&self) -> Option<u32> {
        self.execution.process.lock().unwrap().pid
    }
    /// Observe the root status without driving guest work.
    ///
    /// This can return a status before [`wait_all`](Self::wait_all) completes, while
    /// descendants or stream workers are still alive.
    pub fn try_wait(&self) -> Result<Option<ExitStatus>> {
        self.coordinator.failure()?;
        self.execution.check()?;
        let state = self.execution.process.lock().unwrap();
        Ok(state.status.map(ExitStatus))
    }
    /// Wait for root exit, driving this tree only. Cancellation is safe.
    pub async fn wait(&mut self) -> Result<ExitStatus> {
        self.stdin.take();
        let subscription = self.execution.wake.subscribe();
        poll_fn(|cx| {
            subscription.register(cx.waker());
            if let Some(status) = self.try_wait()? {
                return Poll::Ready(Ok(status));
            }
            let progress = self.coordinator.poll(&self.execution, cx);
            if let Poll::Ready(Err(error)) = progress {
                return Poll::Ready(Err(error));
            }
            match self.try_wait()? {
                Some(status) => Poll::Ready(Ok(status)),
                None => Poll::Pending,
            }
        })
        .await
    }

    /// Wait for the whole process tree and its workers, returning the root's status.
    ///
    /// Cancellation is safe. Descendant exit codes are not aggregated. A
    /// long-lived descendant can keep this wait pending after the root exits;
    /// use [`wait`](Self::wait) to wait for only the root.
    /// Piped output must be drained concurrently or via
    /// [`wait_with_output`](Self::wait_with_output).
    pub async fn wait_all(&mut self) -> Result<ExitStatus> {
        self.stdin.take();
        let subscription = self.execution.wake.subscribe();
        poll_fn(|cx| {
            subscription.register(cx.waker());
            let status = self.try_wait()?;
            if self
                .execution
                .done
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                return Poll::Ready(status.ok_or_else(|| {
                    Error::Wait(io::Error::other("execution ended without a root status"))
                }));
            }
            if let Poll::Ready(Err(error)) = self.coordinator.poll(&self.execution, cx) {
                return Poll::Ready(Err(error));
            }
            Poll::Pending
        })
        .await
    }
    /// Drain both pipes to EOF and collect the root status.
    ///
    /// Descendants may keep pipes open after the root exits. Collected output
    /// is unbounded, like `std::process::Command::output`.
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

struct ChildPipe {
    subscription: Subscription,
    coordinator: Arc<Coordinator>,
    execution: Arc<Execution>,
    stream: usize,
}

impl Drop for ChildPipe {
    fn drop(&mut self) {
        if self.stream == 0 {
            self.execution.pipes[0].eof();
        } else {
            self.execution.pipes[self.stream].close();
        }
    }
}

impl AsyncRead for ChildPipe {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        out: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let pipe = &self.execution.pipes[self.stream];
        self.subscription.register(cx.waker());
        if let Err(error) = self
            .coordinator
            .failure()
            .and_then(|_| self.execution.check())
        {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        if let Poll::Ready(result) = pipe.read(cx, out) {
            return Poll::Ready(result);
        }
        if let Poll::Ready(Err(error)) = self.coordinator.poll(&self.execution, cx) {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        pipe.read(cx, out)
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
        let pipe = &self.0.execution.pipes[0];
        self.0.subscription.register(cx.waker());
        if let Err(error) = self
            .0
            .coordinator
            .failure()
            .and_then(|_| self.0.execution.check())
        {
            return Poll::Ready(Err(io::Error::other(error)));
        }

        if let Poll::Ready(result) = pipe.write(cx, bytes) {
            return Poll::Ready(result);
        }
        if let Poll::Ready(Err(error)) = self.0.coordinator.poll(&self.0.execution, cx) {
            return Poll::Ready(Err(io::Error::other(error)));
        }
        pipe.write(cx, bytes)
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.0.execution.pipes[0].eof();
        Poll::Ready(Ok(()))
    }
}
