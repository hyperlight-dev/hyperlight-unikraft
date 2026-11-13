// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Instant;

use futures::future::{AbortHandle, Abortable, BoxFuture};
use futures::stream::{FuturesUnordered, Stream};

use crate::{CALL_PAYLOAD_MAX, H2G_BUFFER_SIZE, HOST_CALL_MAX, encode_host_reply};

pub(crate) const VERSION: u64 = 1;

pub(crate) fn pending_reply(id: u64) -> Vec<u8> {
    let mut reply = vec![2];
    reply.extend_from_slice(&id.to_le_bytes());
    reply
}

const MAX_PENDING: usize = 1024;
const MAX_BATCH_COUNT: usize = 64;
// Leave a receive buffer for control traffic as well as the framing reserve.
const MAX_BATCH_BYTES: usize = CALL_PAYLOAD_MAX - H2G_BUFFER_SIZE;
const _: () = assert!(8 + 12 + HOST_CALL_MAX <= MAX_BATCH_BYTES);
enum CompletionEvent {
    Call(u64, Vec<u8>),
    Timer(Instant),
    Failed(std::io::Error),
}
type Operation = Abortable<BoxFuture<'static, CompletionEvent>>;

struct Timer {
    deadline: Instant,
    runtime: Option<tokio::runtime::Id>,
    abort: AbortHandle,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Wake {
    Calls,
    Timer,
}
pub(crate) type Handler =
    Arc<dyn Fn(String) -> BoxFuture<'static, Result<String, String>> + Send + Sync>;

#[derive(Default)]
pub(crate) struct Inbox {
    pub negotiated: bool,
    pub interrupt_supported: bool,
    budget: Arc<Mutex<Budget>>,
    outstanding: HashMap<u64, AbortHandle>,
    queued: Vec<(u64, Operation)>,
}

#[derive(Default)]
struct Budget {
    next_id: u64,
    outstanding: usize,
}

impl Inbox {
    pub fn child(&self) -> Self {
        Self {
            negotiated: self.negotiated,
            interrupt_supported: self.interrupt_supported,
            budget: self.budget.clone(),
            outstanding: HashMap::new(),
            queued: Vec::new(),
        }
    }

    pub fn submit(&mut self, name: String, args: String, handler: Handler) -> Result<u64, String> {
        self.submit_reply(async move { encode_host_reply(&name, handler(args).await) })
    }

    pub fn submit_reply(
        &mut self,
        reply: impl std::future::Future<Output = Vec<u8>> + Send + 'static,
    ) -> Result<u64, String> {
        self.submit_fallible_reply(async move { Ok(reply.await) })
    }

    pub fn submit_fallible_reply(
        &mut self,
        reply: impl std::future::Future<Output = std::io::Result<Vec<u8>>> + Send + 'static,
    ) -> Result<u64, String> {
        if !self.negotiated {
            return Err("the kernel does not support async host calls version 1".into());
        }
        let mut budget = self.budget.lock().unwrap();
        if budget.outstanding == MAX_PENDING {
            return Err("too many outstanding async host calls".into());
        }
        let id = budget
            .next_id
            .checked_add(1)
            .ok_or("async request IDs exhausted")?;
        budget.next_id = id;
        budget.outstanding += 1;
        drop(budget);
        let (abort, registration) = AbortHandle::new_pair();
        self.outstanding.insert(id, abort);
        let operation: Operation = Abortable::new(
            Box::pin(async move {
                match reply.await {
                    Ok(reply) if reply.len() <= HOST_CALL_MAX => CompletionEvent::Call(id, reply),
                    Ok(reply) => CompletionEvent::Failed(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!(
                            "async reply exceeds the {HOST_CALL_MAX}-byte limit: {} bytes",
                            reply.len()
                        ),
                    )),
                    Err(error) => CompletionEvent::Failed(error),
                }
            }),
            registration,
        );
        self.queued.push((id, operation));
        Ok(id)
    }

    pub fn is_pending(&self) -> bool {
        !self.outstanding.is_empty()
    }

    pub fn cancel(&mut self, id: u64) {
        if let Some(abort) = self.outstanding.remove(&id) {
            self.budget.lock().unwrap().outstanding -= 1;
            abort.abort();
            self.queued.retain(|(queued_id, _)| *queued_id != id);
        }
    }

    fn delivered(&mut self, id: u64) {
        if self.outstanding.remove(&id).is_some() {
            self.budget.lock().unwrap().outstanding -= 1;
        }
    }

    fn clear(&mut self) {
        self.budget.lock().unwrap().outstanding -= self.outstanding.len();
        self.outstanding.clear();
        self.queued.clear();
    }
}

impl Drop for Inbox {
    fn drop(&mut self) {
        self.clear();
    }
}

#[derive(Default)]
pub(crate) struct PendingCalls {
    pub inbox: Arc<Mutex<Inbox>>,
    running: FuturesUnordered<Operation>,
    batch: Vec<u8>,
    batch_ids: Vec<u64>,
    timer: Option<Timer>,
}

impl PendingCalls {
    pub fn new(inbox: Arc<Mutex<Inbox>>) -> Self {
        Self {
            inbox,
            ..Self::default()
        }
    }

    pub fn is_pending(&self) -> bool {
        self.inbox.lock().unwrap().is_pending()
    }

    /// Only the earliest scheduler deadline matters. Keep an unchanged timer
    /// registered across entries on the same runtime; no runtime is needed until polled.
    pub fn set_timer(&mut self, deadline: Option<Instant>) {
        if self.timer.is_none() && deadline.is_none() {
            return;
        }
        let runtime = tokio::runtime::Handle::try_current().ok().map(|h| h.id());
        if self
            .timer
            .as_ref()
            .is_some_and(|timer| Some(timer.deadline) == deadline && timer.runtime == runtime)
        {
            return;
        }
        if let Some(timer) = self.timer.take() {
            timer.abort.abort();
        }
        if let Some(at) = deadline {
            let (abort, registration) = AbortHandle::new_pair();
            let timer: Operation = Abortable::new(
                Box::pin(async move {
                    tokio::time::sleep_until(at.into()).await;
                    CompletionEvent::Timer(at)
                }),
                registration,
            );
            self.running.push(timer);
            self.timer = Some(Timer {
                deadline: at,
                runtime,
                abort,
            });
        }
    }

    pub fn poll_batch(&mut self, cx: &mut Context<'_>) -> Poll<std::io::Result<Wake>> {
        if !self.batch_ids.is_empty() {
            return Poll::Ready(Ok(Wake::Calls));
        }
        for (_, operation) in self.inbox.lock().unwrap().queued.drain(..) {
            self.running.push(operation);
        }
        self.batch.clear();
        self.batch
            .extend_from_slice(&(VERSION as u32).to_le_bytes());
        self.batch.extend_from_slice(&0u32.to_le_bytes());
        let mut wake = None;
        // Only consume another completion when even a maximum-size reply fits.
        while self.batch_ids.len() < MAX_BATCH_COUNT
            && self.batch.len() + 12 + HOST_CALL_MAX <= MAX_BATCH_BYTES
        {
            let (id, reply) = match std::pin::Pin::new(&mut self.running).poll_next(cx) {
                Poll::Ready(Some(Err(_))) => continue,
                Poll::Ready(Some(Ok(CompletionEvent::Failed(error)))) => {
                    return Poll::Ready(Err(error));
                }
                Poll::Ready(Some(Ok(CompletionEvent::Call(id, reply)))) => (id, reply),
                Poll::Ready(Some(Ok(CompletionEvent::Timer(at)))) => {
                    if self
                        .timer
                        .as_ref()
                        .is_some_and(|timer| timer.deadline == at)
                    {
                        self.timer = None;
                        wake = Some(Wake::Timer);
                    }
                    continue;
                }
                Poll::Ready(None) | Poll::Pending => break,
            };
            if !self.inbox.lock().unwrap().outstanding.contains_key(&id) {
                continue;
            }
            self.batch.extend_from_slice(&id.to_le_bytes());
            self.batch
                .extend_from_slice(&(reply.len() as u32).to_le_bytes());
            self.batch.extend_from_slice(&reply);
            self.batch_ids.push(id);
        }
        if self.batch_ids.is_empty() {
            match wake {
                Some(wake) => Poll::Ready(Ok(wake)),
                None => Poll::Pending,
            }
        } else {
            self.batch[4..8].copy_from_slice(&(self.batch_ids.len() as u32).to_le_bytes());
            // A completion entry runs the scheduler too, servicing an expired
            // timer or ready sockets without a second entry or wire record.
            Poll::Ready(Ok(Wake::Calls))
        }
    }

    pub fn take_batch(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.batch)
    }

    #[cfg(test)]
    fn batch(&self) -> &[u8] {
        &self.batch
    }

    pub fn delivered(&mut self) {
        let mut inbox = self.inbox.lock().unwrap();
        for id in self.batch_ids.drain(..) {
            inbox.delivered(id);
        }
        self.batch.clear();
    }

    pub fn clear(&mut self) {
        self.timer = None;
        self.running.clear();
        self.batch.clear();
        self.batch_ids.clear();
        let mut inbox = self.inbox.lock().unwrap();
        inbox.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{FutureExt, future::poll_fn};
    use std::time::Duration;

    #[tokio::test]
    async fn child_engines_poll_only_their_own_operations() {
        let parent = engine();
        let mut a = PendingCalls::new(Arc::new(Mutex::new(parent.inbox.lock().unwrap().child())));
        let mut b = PendingCalls::new(Arc::new(Mutex::new(parent.inbox.lock().unwrap().child())));
        let (send, receive) = futures::channel::oneshot::channel::<()>();
        let b_id = b
            .inbox
            .lock()
            .unwrap()
            .submit_reply(async move {
                send.send(()).unwrap();
                vec![0]
            })
            .unwrap();
        let a_id = submit(&a, "a");
        assert_ne!(a_id, b_id);
        assert_eq!(poll_fn(|cx| a.poll_batch(cx)).await.unwrap(), Wake::Calls);
        assert!(receive.now_or_never().is_none());
        a.delivered();
        b.inbox.lock().unwrap().cancel(b_id);
        assert!(poll_fn(|cx| b.poll_batch(cx)).now_or_never().is_none());
        assert_eq!(
            parent
                .inbox
                .lock()
                .unwrap()
                .budget
                .lock()
                .unwrap()
                .outstanding,
            0
        );
    }

    #[test]
    fn child_queues_share_capacity_and_release_it_exactly_once() {
        let parent = engine();
        let mut a = parent.inbox.lock().unwrap().child();
        let mut b = a.child();
        let mut first = 0;
        for i in 0..MAX_PENDING {
            let id = (if i % 2 == 0 { &mut a } else { &mut b })
                .submit_reply(std::future::pending())
                .unwrap();
            if i == 0 {
                first = id;
            }
        }
        assert!(a.submit_reply(async { vec![0] }).is_err());
        assert!(b.submit_reply(async { vec![0] }).is_err());
        a.cancel(first);
        a.cancel(first);
        b.submit_reply(async { vec![0] }).unwrap();
        assert!(a.submit_reply(async { vec![0] }).is_err());
        drop(a);
        drop(b);
        let budget = parent.inbox.lock().unwrap().budget.clone();
        assert_eq!(budget.lock().unwrap().outstanding, 0);
        budget.lock().unwrap().next_id = u64::MAX;
        assert!(
            parent
                .inbox
                .lock()
                .unwrap()
                .submit_reply(async { vec![0] })
                .is_err()
        );
        assert_eq!(budget.lock().unwrap().outstanding, 0);
    }

    #[tokio::test]
    async fn timer_replaces_retains_and_cancels_deadlines() {
        let mut engine = engine();
        let later = Instant::now() + Duration::from_secs(60);
        engine.set_timer(Some(later));
        engine.set_timer(Some(later));
        assert_eq!(engine.running.len(), 1);
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        let sooner = Instant::now() + Duration::from_millis(10);
        engine.set_timer(Some(sooner));
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), poll_fn(|cx| engine.poll_batch(cx)))
                .await
                .unwrap()
                .unwrap(),
            Wake::Timer
        );
        assert!(engine.timer.is_none());
        assert!(
            !engine.is_pending(),
            "timers are not outstanding host calls"
        );

        engine.set_timer(Some(Instant::now()));
        engine.set_timer(None);
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
    }

    #[tokio::test]
    async fn later_deadline_suppresses_old_expiry_and_clear_drops_timer() {
        let mut engine = engine();
        engine.set_timer(Some(Instant::now()));
        engine.set_timer(Some(Instant::now() + Duration::from_secs(60)));
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        engine.clear();
        assert!(engine.timer.is_none());
        assert!(engine.running.is_empty());
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
    }

    #[tokio::test]
    async fn expired_timer_counts_time_spent_outside_driver() {
        let mut engine = engine();
        let deadline = Instant::now();
        // Register only after the deadline: the sleep must not start a new delay.
        tokio::time::sleep(Duration::from_millis(10)).await;
        engine.set_timer(Some(deadline));
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), poll_fn(|cx| engine.poll_batch(cx)))
                .await
                .unwrap()
                .unwrap(),
            Wake::Timer
        );
        assert!(engine.batch_ids.is_empty());
    }

    #[tokio::test]
    async fn timer_and_ready_calls_share_a_wake_without_starvation() {
        let mut engine = engine();
        engine.set_timer(Some(Instant::now() + Duration::from_secs(60)));
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        engine.set_timer(Some(Instant::now() - Duration::from_millis(10)));
        for _ in 0..130 {
            submit(&engine, "ready");
        }
        let mut count = 0;
        for _ in 0..3 {
            assert_eq!(
                poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
                Wake::Calls
            );
            count += engine.batch_ids.len();
            engine.delivered();
        }
        assert_eq!(count, 130);
        assert!(engine.timer.is_none());
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
    }

    #[tokio::test]
    async fn host_call_completion_retains_unchanged_timer() {
        let mut engine = engine();
        let deadline = Instant::now() + Duration::from_secs(60);
        engine.set_timer(Some(deadline));
        submit(&engine, "ready");
        assert_eq!(
            poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
            Wake::Calls
        );
        engine.delivered();
        engine.set_timer(Some(deadline));
        assert_eq!(engine.running.len(), 1);
        assert_eq!(
            engine.timer.as_ref().map(|timer| timer.deadline),
            Some(deadline)
        );
        assert!(!engine.is_pending());
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
    }

    #[tokio::test]
    async fn timer_wakes_while_host_call_remains_pending() {
        let mut engine = engine();
        engine
            .inbox
            .lock()
            .unwrap()
            .submit(
                "pending".into(),
                String::new(),
                crate::closure_handler(|_| futures::future::pending().boxed()),
            )
            .unwrap();
        engine.set_timer(Some(Instant::now() + Duration::from_millis(10)));
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), poll_fn(|cx| engine.poll_batch(cx)))
                .await
                .unwrap()
                .unwrap(),
            Wake::Timer
        );
        assert!(engine.is_pending());
        assert!(engine.batch_ids.is_empty());
    }

    fn engine() -> PendingCalls {
        let engine = PendingCalls::default();
        engine.inbox.lock().unwrap().negotiated = true;
        engine
    }

    #[test]
    fn retained_timer_moves_to_a_new_runtime() {
        let mut engine = engine();
        let deadline = Instant::now() + Duration::from_millis(50);
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            engine.set_timer(Some(deadline));
            assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        });
        drop(runtime);
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            engine.set_timer(Some(deadline));
            assert_eq!(
                tokio::time::timeout(Duration::from_secs(1), poll_fn(|cx| engine.poll_batch(cx)))
                    .await
                    .unwrap()
                    .unwrap(),
                Wake::Timer
            );
            assert!(engine.running.is_empty());
        });
    }

    #[tokio::test]
    async fn saturated_calls_cancel_refill_and_deliver_once() {
        use std::collections::HashSet;

        let mut engine = engine();
        let mut previous = 0;
        for _ in 0..8 {
            let mut pending = Vec::new();
            for _ in 0..MAX_PENDING {
                let (send, recv) = futures::channel::oneshot::channel();
                let id = engine
                    .inbox
                    .lock()
                    .unwrap()
                    .submit_reply(async move { recv.await.unwrap() })
                    .unwrap();
                assert!(id > previous);
                previous = id;
                pending.push((id, send));
            }
            assert!(
                engine
                    .inbox
                    .lock()
                    .unwrap()
                    .submit_reply(async { vec![0] })
                    .is_err()
            );
            assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
            assert_eq!(engine.running.len(), MAX_PENDING);

            for (i, (id, _)) in pending.iter().enumerate() {
                if i % 4 == 0 {
                    engine.inbox.lock().unwrap().cancel(*id);
                }
            }
            assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
            assert_eq!(engine.running.len(), MAX_PENDING * 3 / 4);
            let mut expected = HashSet::new();
            for (i, (id, send)) in pending.into_iter().enumerate() {
                if i % 4 == 0 {
                    assert!(send.is_canceled());
                    let replacement = submit(&engine, "replacement");
                    assert!(replacement > previous);
                    previous = replacement;
                    expected.insert(replacement);
                } else {
                    expected.insert(id);
                    send.send(vec![0, b'7']).unwrap();
                }
            }
            assert_eq!(expected.len(), MAX_PENDING);
            let mut batches = 0;
            while !expected.is_empty() {
                assert_eq!(
                    poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
                    Wake::Calls
                );
                assert_eq!(engine.batch_ids.len(), MAX_BATCH_COUNT);
                assert!(engine.batch.len() <= MAX_BATCH_BYTES);
                assert_eq!(
                    u32::from_le_bytes(engine.batch[4..8].try_into().unwrap()) as usize,
                    MAX_BATCH_COUNT
                );
                for id in &engine.batch_ids {
                    assert!(expected.remove(id), "duplicate or cancelled ID {id}");
                }
                engine.delivered();
                batches += 1;
            }
            assert_eq!(batches, MAX_PENDING / MAX_BATCH_COUNT);
            assert!(!engine.is_pending());
            assert!(engine.running.is_empty());
            assert!(engine.inbox.lock().unwrap().queued.is_empty());
            engine.clear();
        }
    }

    #[tokio::test]
    async fn repeated_clear_drops_saturated_work_in_every_phase() {
        let mut engine = engine();
        let mut previous = 0;
        for phase in [0, 1, 2].into_iter().cycle().take(12) {
            let mut senders = Vec::new();
            for _ in 0..MAX_PENDING {
                let (send, recv) = futures::channel::oneshot::channel();
                let id = engine
                    .inbox
                    .lock()
                    .unwrap()
                    .submit_reply(async move { recv.await.unwrap() })
                    .unwrap();
                assert!(id > previous);
                previous = id;
                senders.push(send);
            }
            if phase > 0 {
                assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
            }
            if phase == 2 {
                for send in senders.drain(..MAX_BATCH_COUNT) {
                    send.send(vec![0, b'7']).unwrap();
                }
                assert_eq!(
                    poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
                    Wake::Calls
                );
                assert_eq!(engine.batch_ids.len(), MAX_BATCH_COUNT);
            }
            engine.clear();
            assert!(senders.iter().all(|send| send.is_canceled()));
            assert!(!engine.is_pending());
            assert!(engine.running.is_empty());
            assert!(engine.batch.is_empty());
            assert!(engine.batch_ids.is_empty());
            assert!(engine.inbox.lock().unwrap().queued.is_empty());
            assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        }
    }

    fn submit(engine: &PendingCalls, reply: &str) -> u64 {
        let reply = reply.to_string();
        engine
            .inbox
            .lock()
            .unwrap()
            .submit(
                "test".into(),
                String::new(),
                crate::closure_handler(move |_| futures::future::ready(Ok(reply.clone())).boxed()),
            )
            .unwrap()
    }

    #[tokio::test]
    async fn wire_vector_and_refill_after_empty() {
        let mut engine = engine();
        assert_eq!(submit(&engine, "7"), 1);
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(
            engine.batch(),
            &[
                1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, b'7',
            ]
        );
        assert!(engine.is_pending());
        engine.delivered();
        assert!(!engine.is_pending());
        assert_eq!(submit(&engine, "8"), 2);
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(*engine.batch().last().unwrap(), b'8');
    }

    #[tokio::test]
    async fn out_of_order_and_cancelled_driver_preserve_work() {
        let mut engine = engine();
        let (send, recv) = futures::channel::oneshot::channel();
        let recv = Arc::new(Mutex::new(Some(recv)));
        engine
            .inbox
            .lock()
            .unwrap()
            .submit(
                "slow".into(),
                String::new(),
                crate::closure_handler(move |_| {
                    let recv = recv.lock().unwrap().take().unwrap();
                    async move { recv.await.map_err(|e| e.to_string()) }.boxed()
                }),
            )
            .unwrap();
        // Poll once and drop the driving future.
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        submit(&engine, "fast");
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(
            u64::from_le_bytes(engine.batch()[8..16].try_into().unwrap()),
            2
        );
        let batch = engine.batch().to_vec();
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(engine.batch(), batch); // retained until acknowledged
        engine.delivered();
        send.send("slow".into()).unwrap();
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(
            u64::from_le_bytes(engine.batch()[8..16].try_into().unwrap()),
            1
        );
    }

    #[tokio::test]
    async fn batching_is_bounded_by_count_and_bytes() {
        let mut engine = engine();
        for _ in 0..65 {
            submit(&engine, "x");
        }
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(engine.batch_ids.len(), 64);
        engine.delivered();
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(engine.batch_ids.len(), 1);
        engine.delivered();
        let per_batch = (MAX_BATCH_BYTES - 8) / (12 + HOST_CALL_MAX);
        assert!(per_batch > 1);
        for _ in 0..2 * per_batch + 1 {
            submit(&engine, &"x".repeat(HOST_CALL_MAX - 1));
        }
        for count in [per_batch, per_batch, 1] {
            poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
            assert_eq!(engine.batch_ids.len(), count);
            assert_eq!(engine.batch.len(), 8 + count * (12 + HOST_CALL_MAX));
            assert!(engine.batch.len() <= MAX_BATCH_BYTES);
            if count == per_batch {
                assert!(engine.batch.len() + 12 + HOST_CALL_MAX > MAX_BATCH_BYTES);
            }
            engine.delivered();
        }
        assert!(!engine.is_pending());
    }

    #[tokio::test]
    async fn mixed_replies_can_fill_the_exact_transport_budget() {
        let mut engine = engine();
        let full = (MAX_BATCH_BYTES - 8) / (12 + HOST_CALL_MAX);
        let padding = (MAX_BATCH_BYTES - 8) % (12 + HOST_CALL_MAX);
        assert!(padding > 12);
        let mut senders = Vec::new();
        for _ in 0..full {
            let (send, receive) = futures::channel::oneshot::channel();
            senders.push(send);
            engine
                .inbox
                .lock()
                .unwrap()
                .submit_reply(async move { receive.await.unwrap() })
                .unwrap();
        }
        engine
            .inbox
            .lock()
            .unwrap()
            .submit_reply(async move {
                for send in senders {
                    send.send(vec![0; HOST_CALL_MAX]).unwrap();
                }
                vec![0; padding - 12]
            })
            .unwrap();
        assert_eq!(
            poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
            Wake::Calls
        );
        assert_eq!(engine.batch.len(), MAX_BATCH_BYTES);
        assert_eq!(engine.batch_ids.len(), full + 1);
        engine.delivered();
        assert!(!engine.is_pending());
        assert!(engine.running.is_empty());
    }

    #[tokio::test]
    async fn cancellation_and_clear_discard_unbatched_completions() {
        for cancel in [true, false] {
            let mut engine = engine();
            let per_batch = (MAX_BATCH_BYTES - 8) / (12 + HOST_CALL_MAX);
            let ids: Vec<_> = (0..=per_batch)
                .map(|_| submit(&engine, &"x".repeat(HOST_CALL_MAX - 1)))
                .collect();
            assert_eq!(
                poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
                Wake::Calls
            );
            let remaining: Vec<_> = ids
                .into_iter()
                .filter(|id| !engine.batch_ids.contains(id))
                .collect();
            assert_eq!(remaining.len(), 1);
            assert_eq!(engine.running.len(), 1);
            let batch = engine.take_batch();
            assert_eq!(batch.len(), 8 + per_batch * (12 + HOST_CALL_MAX));
            assert_eq!(
                u32::from_le_bytes(batch[4..8].try_into().unwrap()) as usize,
                per_batch
            );
            assert_eq!(engine.batch_ids.len(), per_batch);
            if cancel {
                engine.inbox.lock().unwrap().cancel(remaining[0]);
                engine.delivered();
                assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
            } else {
                engine.clear();
            }
            assert!(engine.running.is_empty());
            assert!(engine.batch_ids.is_empty());
            assert!(!engine.is_pending());

            let id = submit(&engine, "next");
            assert_eq!(
                poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
                Wake::Calls
            );
            assert_eq!(engine.batch_ids, [id]);
            assert_eq!(
                u32::from_le_bytes(engine.batch()[4..8].try_into().unwrap()),
                1
            );
            engine.delivered();
            assert!(!engine.is_pending());
        }
    }

    #[tokio::test]
    async fn oversized_internal_reply_fails_instead_of_exceeding_the_batch_budget() {
        let mut engine = engine();
        engine
            .inbox
            .lock()
            .unwrap()
            .submit_reply(async { vec![0; HOST_CALL_MAX + 1] })
            .unwrap();
        let error = poll_fn(|cx| engine.poll_batch(cx)).await.unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert!(engine.batch_ids.is_empty());
        engine.clear();
        assert!(!engine.is_pending());
    }

    #[tokio::test]
    async fn errors_limits_and_clear() {
        let mut engine = engine();
        submit(&engine, &"x".repeat(HOST_CALL_MAX));
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(engine.batch()[20], crate::HOST_CALL_ERR);
        engine.clear();
        assert!(!engine.is_pending());
        let previous = engine.inbox.lock().unwrap().budget.lock().unwrap().next_id;
        assert!(submit(&engine, "next") > previous);
        engine.clear();
        let mut inbox = engine.inbox.lock().unwrap();
        inbox.negotiated = false;
        let f: Handler =
            crate::closure_handler(|_| futures::future::ready(Ok(String::new())).boxed());
        assert!(inbox.submit("f".into(), String::new(), f.clone()).is_err());
        inbox.negotiated = true;
        inbox.budget.lock().unwrap().next_id = u64::MAX;
        assert!(inbox.submit("f".into(), String::new(), f.clone()).is_err());
        inbox.budget.lock().unwrap().next_id = 0;
        for id in 1..=MAX_PENDING as u64 {
            inbox.outstanding.insert(id, AbortHandle::new_pair().0);
        }
        inbox.budget.lock().unwrap().outstanding = MAX_PENDING;
        assert!(inbox.submit("f".into(), String::new(), f).is_err());
    }

    #[tokio::test]
    async fn cancellation_drops_running_future_without_delivery() {
        let mut engine = engine();
        let (send, recv) = futures::channel::oneshot::channel::<String>();
        let recv = Arc::new(Mutex::new(Some(recv)));
        let id = engine
            .inbox
            .lock()
            .unwrap()
            .submit(
                "wait".into(),
                String::new(),
                crate::closure_handler(move |_| {
                    let recv = recv.lock().unwrap().take().unwrap();
                    async move { recv.await.map_err(|e| e.to_string()) }.boxed()
                }),
            )
            .unwrap();
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        engine.inbox.lock().unwrap().cancel(id);
        engine.inbox.lock().unwrap().cancel(id);
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        assert!(!engine.is_pending());
        assert!(send.is_canceled());
    }

    #[test]
    fn cancelling_queued_work_releases_it_without_polling() {
        let engine = engine();
        let id = submit(&engine, "unused");
        let mut inbox = engine.inbox.lock().unwrap();
        inbox.cancel(id);
        assert!(inbox.queued.is_empty());
        assert!(!inbox.is_pending());
    }

    #[tokio::test]
    async fn clear_drops_work_and_does_not_acknowledge_new_ids() {
        let mut engine = engine();
        submit(&engine, "before");
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        engine.clear();
        submit(&engine, "after");
        poll_fn(|cx| engine.poll_batch(cx)).await.unwrap();
        assert_eq!(engine.batch_ids, [2]);
        engine.delivered();
        assert!(!engine.is_pending());
    }
}
