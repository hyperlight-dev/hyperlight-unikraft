// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Where a sandbox's time goes, seen from the host: each VM entry, split
//! into time in the guest and time in the host functions it calls (one VM
//! exit each), each host function, and Hyperlight's own restore and boot.
//!
//! Off unless `HLUK_PROFILE` is set (to anything but `0`) or
//! [`SandboxBuilder::profile`](crate::SandboxBuilder::profile) asks for it;
//! off, it costs one atomic load per entry and host function.

use std::collections::BTreeMap;
use std::fmt::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A sandbox's profile: [`report`](Self::report) prints it,
/// [`reset`](Self::reset) starts it over (after a warm-up, say).
pub struct Profile {
    enabled: AtomicBool,
    data: Mutex<Data>,
}

#[derive(Default)]
struct Data {
    rows: BTreeMap<String, Row>,
    /// VM exits (host function calls) per entry kind.
    exits: BTreeMap<String, u64>,
    /// Host function time and exits within the entry in progress.
    entry_host: Duration,
    entry_exits: u64,
}

#[derive(Default, Clone, Copy)]
struct Row {
    count: u64,
    total: Duration,
    max: Duration,
}

impl Row {
    fn add(&mut self, d: Duration) {
        self.count += 1;
        self.total += d;
        self.max = self.max.max(d);
    }
}

impl Profile {
    pub(crate) fn new() -> Arc<Self> {
        let on = std::env::var_os("HLUK_PROFILE").is_some_and(|v| v != "0");
        Arc::new(Self {
            enabled: AtomicBool::new(on),
            data: Mutex::new(Data::default()),
        })
    }

    pub(crate) fn set_enabled(&self, on: bool) {
        self.enabled.store(on, Ordering::Relaxed);
    }

    /// Whether this sandbox is being profiled.
    pub fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Time `f` under `label` when enabled.
    pub(crate) fn time<T>(&self, label: &str, f: impl FnOnce() -> T) -> T {
        if !self.enabled() {
            return f();
        }
        let start = Instant::now();
        let out = f();
        self.record(label, start.elapsed());
        out
    }

    pub(crate) fn record(&self, label: &str, d: Duration) {
        if !self.enabled() {
            return;
        }
        self.data
            .lock()
            .unwrap()
            .rows
            .entry(label.to_string())
            .or_default()
            .add(d);
    }

    /// A guard timing one host function call, which is one VM exit.
    pub(crate) fn host(self: &Arc<Self>, name: &'static str) -> Option<HostGuard> {
        self.enabled().then(|| HostGuard {
            profile: self.clone(),
            name,
            start: Instant::now(),
        })
    }

    /// Run one VM entry (`f`) as `name`: its wall time, and within it the
    /// host function time, the rest being the guest's (VM exits and entry
    /// included).
    pub(crate) fn entry<T>(&self, name: &str, f: impl FnOnce() -> T) -> T {
        if !self.enabled() {
            return f();
        }
        {
            let mut data = self.data.lock().unwrap();
            data.entry_host = Duration::ZERO;
            data.entry_exits = 0;
        }
        let start = Instant::now();
        let out = f();
        let total = start.elapsed();
        let mut data = self.data.lock().unwrap();
        let (host, exits) = (data.entry_host, data.entry_exits);
        data.rows
            .entry(format!("entry {name}"))
            .or_default()
            .add(total);
        data.rows
            .entry(format!("entry {name} / guest"))
            .or_default()
            .add(total.saturating_sub(host));
        data.rows
            .entry(format!("entry {name} / host functions"))
            .or_default()
            .add(host);
        *data.exits.entry(format!("entry {name}")).or_default() += exits;
        out
    }

    /// Start over.
    pub fn reset(&self) {
        *self.data.lock().unwrap() = Data::default();
    }

    /// The profile as a table: entries (with their guest and host function
    /// time and exits per entry), then the rest by total time.
    pub fn report(&self) -> String {
        let data = self.data.lock().unwrap();
        let mut out = String::new();
        let _ = writeln!(
            out,
            "{:<40} {:>8} {:>11} {:>10} {:>10} {:>8}",
            "hluk profile (host side)", "count", "total ms", "mean µs", "max µs", "exits"
        );
        let line = |out: &mut String, label: &str, row: &Row, exits: Option<f64>| {
            let mean = row.total.as_secs_f64() * 1e6 / row.count.max(1) as f64;
            let _ = writeln!(
                out,
                "{:<40} {:>8} {:>11.3} {:>10.1} {:>10.1} {:>8}",
                label,
                row.count,
                row.total.as_secs_f64() * 1e3,
                mean,
                row.max.as_secs_f64() * 1e6,
                exits.map_or(String::new(), |e| format!("{e:.1}")),
            );
        };
        let mut entries: Vec<_> = data
            .rows
            .iter()
            .filter(|(k, _)| k.starts_with("entry ") && !k.contains(" / "))
            .collect();
        entries.sort_by_key(|(_, row)| std::cmp::Reverse(row.total));
        for (label, row) in entries {
            let exits = data
                .exits
                .get(label)
                .map(|&e| e as f64 / row.count.max(1) as f64);
            line(&mut out, label, row, exits);
            for part in ["guest", "host functions"] {
                if let Some(r) = data.rows.get(&format!("{label} / {part}")) {
                    line(&mut out, &format!("  {part}"), r, None);
                }
            }
        }
        let mut rest: Vec<_> = data
            .rows
            .iter()
            .filter(|(k, _)| !k.starts_with("entry "))
            .collect();
        rest.sort_by_key(|(_, row)| std::cmp::Reverse(row.total));
        for (label, row) in rest {
            line(&mut out, label, row, None);
        }
        out
    }
}

/// Times one host function call; recorded when dropped.
pub(crate) struct HostGuard {
    profile: Arc<Profile>,
    name: &'static str,
    start: Instant,
}

impl Drop for HostGuard {
    fn drop(&mut self) {
        let d = self.start.elapsed();
        let mut data = self.profile.data.lock().unwrap();
        data.entry_host += d;
        data.entry_exits += 1;
        data.rows
            .entry(format!("host {}", self.name))
            .or_default()
            .add(d);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn on() -> Arc<Profile> {
        let p = Profile::new();
        p.set_enabled(true);
        p
    }

    fn position(report: &str, label: &str) -> usize {
        report
            .lines()
            .position(|l| l.starts_with(label))
            .unwrap_or_else(|| panic!("no {label:?} in\n{report}"))
    }

    #[test]
    fn off_records_nothing() {
        let p = on();
        p.set_enabled(false);
        p.record("restore: hyperlight", Duration::from_millis(1));
        assert!(p.host("GetTscHz").is_none());
        assert_eq!(p.entry("Call", || 7), 7);
        assert_eq!(p.time("boot", || 8), 8);
        assert_eq!(p.report().lines().count(), 1, "only the header");
    }

    #[test]
    fn an_entry_splits_guest_and_host_time_and_counts_its_exits() {
        let p = on();
        for _ in 0..2 {
            p.entry("Call", || {
                drop(p.host("Yield"));
                drop(p.host("GetEnvVars"));
                drop(p.host("Yield"));
            });
        }
        let data = p.data.lock().unwrap();
        let (entry, guest, host) = (
            data.rows["entry Call"],
            data.rows["entry Call / guest"],
            data.rows["entry Call / host functions"],
        );
        assert_eq!((entry.count, guest.count, host.count), (2, 2, 2));
        assert_eq!(guest.total + host.total, entry.total);
        assert_eq!(data.exits["entry Call"], 6);
        assert_eq!(data.rows["host Yield"].count, 4);
        assert_eq!(data.rows["host GetEnvVars"].count, 2);
        drop(data);
        let report = p.report();
        let call = report
            .lines()
            .find(|l| l.starts_with("entry Call"))
            .unwrap();
        assert!(call.ends_with("3.0"), "3 exits per entry: {call}");
    }

    #[test]
    fn the_report_lists_entries_first_then_the_rest_by_total() {
        let p = on();
        p.entry("resume", || drop(p.host("GetResumeState")));
        p.record("entry Exec", Duration::from_secs(5));
        p.record("host A", Duration::from_millis(1));
        p.record("host B", Duration::from_millis(9));
        let report = p.report();
        assert!(position(&report, "entry Exec") < position(&report, "entry resume"));
        assert_eq!(
            position(&report, "  guest"),
            position(&report, "entry resume") + 1
        );
        assert!(position(&report, "entry resume") < position(&report, "host B"));
        assert!(position(&report, "host B") < position(&report, "host A"));
    }

    #[test]
    fn reset_starts_over() {
        let p = on();
        p.record("restore: hyperlight", Duration::from_millis(1));
        p.reset();
        assert_eq!(p.report().lines().count(), 1);
        p.record("restore: hyperlight", Duration::from_millis(1));
        assert_eq!(p.report().lines().count(), 2);
    }
}
