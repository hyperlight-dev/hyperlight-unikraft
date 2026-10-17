// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use futures::future::{AbortHandle, Abortable, BoxFuture};
use futures::stream::{FuturesUnordered, Stream};

use crate::{HOST_CALL_MAX, encode_host_reply};

pub(crate) const VERSION: u64 = 1;
const MAX_PENDING: usize = 1024;
const MAX_BATCH_COUNT: usize = 64;
const MAX_BATCH_BYTES: usize = HOST_CALL_MAX + 20;
type Completion = (u64, Vec<u8>);
type Operation = BoxFuture<'static, Option<Completion>>;
pub(crate) type Handler =
    Arc<dyn Fn(String) -> BoxFuture<'static, Result<String, String>> + Send + Sync>;

#[derive(Default)]
pub(crate) struct Inbox {
    pub negotiated: bool,
    next_id: u64,
    outstanding: HashMap<u64, AbortHandle>,
    queued: Vec<(u64, Operation)>,
}

impl Inbox {
    pub fn submit(&mut self, name: String, args: String, handler: Handler) -> Result<u64, String> {
        if !self.negotiated {
            return Err("the kernel does not support async host calls version 1".into());
        }
        if self.outstanding.len() == MAX_PENDING {
            return Err("too many outstanding async host calls".into());
        }
        let id = self
            .next_id
            .checked_add(1)
            .ok_or("async request IDs exhausted")?;
        self.next_id = id;
        let (abort, registration) = AbortHandle::new_pair();
        self.outstanding.insert(id, abort);
        let operation = Abortable::new(
            async move {
                let result = handler(args).await;
                (id, encode_host_reply(&name, result))
            },
            registration,
        );
        self.queued
            .push((id, Box::pin(async move { operation.await.ok() })));
        Ok(id)
    }

    pub fn is_pending(&self) -> bool {
        !self.outstanding.is_empty()
    }

    pub fn cancel(&mut self, id: u64) {
        if let Some(abort) = self.outstanding.remove(&id) {
            abort.abort();
            self.queued.retain(|(queued_id, _)| *queued_id != id);
        }
    }
}

#[derive(Default)]
pub(crate) struct PendingCalls {
    pub inbox: Arc<Mutex<Inbox>>,
    running: FuturesUnordered<Operation>,
    ready: VecDeque<Completion>,
    batch: Vec<u8>,
    batch_count: usize,
    batch_ids: Vec<u64>,
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

    pub fn poll_batch(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if self.batch_count != 0 {
            return Poll::Ready(());
        }
        for (_, operation) in self.inbox.lock().unwrap().queued.drain(..) {
            self.running.push(operation);
        }
        self.batch.clear();
        self.batch
            .extend_from_slice(&(VERSION as u32).to_le_bytes());
        self.batch.extend_from_slice(&0u32.to_le_bytes());
        while self.batch_count < MAX_BATCH_COUNT {
            let next = self.ready.pop_front().or_else(|| {
                loop {
                    match std::pin::Pin::new(&mut self.running).poll_next(cx) {
                        Poll::Ready(Some(None)) => continue,
                        Poll::Ready(Some(Some(next))) => break Some(next),
                        Poll::Ready(None) | Poll::Pending => break None,
                    }
                }
            });
            let Some((id, reply)) = next else { break };
            if !self.inbox.lock().unwrap().outstanding.contains_key(&id) {
                continue;
            }
            if self.batch.len() + 12 + reply.len() > MAX_BATCH_BYTES {
                self.ready.push_front((id, reply));
                break;
            }
            self.batch.extend_from_slice(&id.to_le_bytes());
            self.batch
                .extend_from_slice(&(reply.len() as u32).to_le_bytes());
            self.batch.extend_from_slice(&reply);
            self.batch_count += 1;
            self.batch_ids.push(id);
        }
        if self.batch_count == 0 {
            Poll::Pending
        } else {
            self.batch[4..8].copy_from_slice(&(self.batch_count as u32).to_le_bytes());
            Poll::Ready(())
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
            inbox.outstanding.remove(&id);
        }
        self.batch_count = 0;
        self.batch.clear();
    }

    pub fn clear(&mut self) {
        self.running.clear();
        self.ready.clear();
        self.batch.clear();
        self.batch_count = 0;
        self.batch_ids.clear();
        let mut inbox = self.inbox.lock().unwrap();
        inbox.queued.clear();
        inbox.outstanding.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{FutureExt, future::poll_fn};

    fn engine() -> PendingCalls {
        let engine = PendingCalls::default();
        engine.inbox.lock().unwrap().negotiated = true;
        engine
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
                Arc::new(move |_| futures::future::ready(Ok(reply.clone())).boxed()),
            )
            .unwrap()
    }

    #[tokio::test]
    async fn wire_vector_and_refill_after_empty() {
        let mut engine = engine();
        assert_eq!(submit(&engine, "7"), 1);
        poll_fn(|cx| engine.poll_batch(cx)).await;
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
        poll_fn(|cx| engine.poll_batch(cx)).await;
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
                Arc::new(move |_| {
                    let recv = recv.lock().unwrap().take().unwrap();
                    async move { recv.await.map_err(|e| e.to_string()) }.boxed()
                }),
            )
            .unwrap();
        // Poll once and drop the driving future.
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        submit(&engine, "fast");
        poll_fn(|cx| engine.poll_batch(cx)).await;
        assert_eq!(
            u64::from_le_bytes(engine.batch()[8..16].try_into().unwrap()),
            2
        );
        let batch = engine.batch().to_vec();
        poll_fn(|cx| engine.poll_batch(cx)).await;
        assert_eq!(engine.batch(), batch); // retained until acknowledged
        engine.delivered();
        send.send("slow".into()).unwrap();
        poll_fn(|cx| engine.poll_batch(cx)).await;
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
        poll_fn(|cx| engine.poll_batch(cx)).await;
        assert_eq!(engine.batch_count, 64);
        engine.delivered();
        poll_fn(|cx| engine.poll_batch(cx)).await;
        assert_eq!(engine.batch_count, 1);
        engine.delivered();
        for _ in 0..2 {
            submit(&engine, &"x".repeat(HOST_CALL_MAX - 1));
        }
        for _ in 0..2 {
            poll_fn(|cx| engine.poll_batch(cx)).await;
            assert_eq!(engine.batch_count, 1);
            assert_eq!(engine.batch.len(), MAX_BATCH_BYTES);
            engine.delivered();
        }
        assert!(!engine.is_pending());
    }

    #[tokio::test]
    async fn errors_limits_and_clear() {
        let mut engine = engine();
        submit(&engine, &"x".repeat(HOST_CALL_MAX));
        poll_fn(|cx| engine.poll_batch(cx)).await;
        assert_eq!(engine.batch()[20], crate::HOST_CALL_ERR);
        engine.clear();
        assert!(!engine.is_pending());
        let previous = engine.inbox.lock().unwrap().next_id;
        assert!(submit(&engine, "next") > previous);
        engine.clear();
        let mut inbox = engine.inbox.lock().unwrap();
        inbox.negotiated = false;
        let f: Handler = Arc::new(|_| futures::future::ready(Ok(String::new())).boxed());
        assert!(inbox.submit("f".into(), String::new(), f.clone()).is_err());
        inbox.negotiated = true;
        inbox.next_id = u64::MAX;
        assert!(inbox.submit("f".into(), String::new(), f.clone()).is_err());
        inbox.next_id = 0;
        for id in 1..=MAX_PENDING as u64 {
            inbox.outstanding.insert(id, AbortHandle::new_pair().0);
        }
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
                Arc::new(move |_| {
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
        poll_fn(|cx| engine.poll_batch(cx)).await;
        engine.clear();
        submit(&engine, "after");
        poll_fn(|cx| engine.poll_batch(cx)).await;
        assert_eq!(engine.batch_ids, [2]);
        engine.delivered();
        assert!(!engine.is_pending());
    }
}
