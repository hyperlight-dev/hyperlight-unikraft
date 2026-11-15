// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! What the fuzz targets in `fuzz/` drive: the `fs_*` and `net_*` host
//! functions, called as Hyperlight calls them for a guest, and the DNS
//! parsers the network policy reads guest and resolver messages with.
//! Behind the `fuzzing` feature, and not a stable API.
//!
//! A guest's host call reaches Hyperlight as a FlatBuffer, which
//! hyperlight-common decodes (and Hyperlight's own fuzz targets fuzz) into
//! the function's name and a list of [`ParameterValue`]s.  Hyperlight
//! then looks the function up by name and converts the list to the
//! function's parameter types, refusing a list of the wrong length or
//! types.  [`HostCalls::call`] starts from that list and does the same,
//! so every value a guest can put in one reaches the handlers here as it
//! would in a sandbox, with no VM in the way.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use hyperlight_host::func::{
    HostFunction, ParameterTuple, ParameterType, ParameterValue, Registerable, ReturnValue,
    SupportedReturnType,
};
use hyperlight_host::new_error;

use crate::{ListenPorts, Mount, NetworkPolicy, hostfs, hostnet, net_policy, profile};

/// The longest reply one host call carries, which every reply must fit.
pub const HOST_CALL_MAX: usize = crate::HOST_CALL_MAX;

/// The most bytes `fs_read_bytes` reads in one call.
pub const FS_CHUNK: usize = hostfs::CHUNK;

type Call = dyn Fn(Vec<ParameterValue>) -> hyperlight_host::Result<ReturnValue> + Send + Sync;

/// A registered host function: what Hyperlight's own registry keeps.
struct Entry {
    parameter_types: &'static [ParameterType],
    call: Box<Call>,
}

/// The `fs_*` and `net_*` host functions of one sandbox, by name.
pub struct HostCalls {
    functions: HashMap<String, Entry>,
    net: Arc<hostnet::Net>,
}

impl Registerable for HostCalls {
    fn register_host_function<Args: ParameterTuple, Output: SupportedReturnType>(
        &mut self,
        name: &str,
        hf: impl Into<HostFunction<Output, Args>>,
    ) -> hyperlight_host::Result<()> {
        let hf = hf.into();
        let call = move |args: Vec<ParameterValue>| -> hyperlight_host::Result<ReturnValue> {
            Ok(hf.call(Args::from_value(args)?)?.into_value())
        };
        self.functions.insert(
            name.to_string(),
            Entry {
                parameter_types: Args::TYPE,
                call: Box::new(call),
            },
        );
        Ok(())
    }
}

impl HostCalls {
    /// The host functions a sandbox with these mounts and this network
    /// gets: no policy is no network, every `socket()` refused, as for a
    /// sandbox built without one.
    pub fn new(
        mounts: &[Mount],
        network: Option<NetworkPolicy>,
        listen_ports: Option<ListenPorts>,
    ) -> crate::Result<Self> {
        let net = Arc::new(match network {
            Some(policy) => hostnet::Net::new(policy, listen_ports),
            None => hostnet::Net::disabled(),
        });
        let mut calls = Self {
            functions: HashMap::new(),
            net: net.clone(),
        };
        let profile = profile::Profile::new();
        hostfs::register(&mut calls, mounts, &profile)?;
        hostnet::register(&mut calls, &net, &profile)?;
        Ok(calls)
    }

    /// The names of the host functions, in no particular order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.functions.keys().map(String::as_str)
    }

    /// The parameter types of host function `name`.
    pub fn parameter_types(&self, name: &str) -> Option<&'static [ParameterType]> {
        self.functions.get(name).map(|e| e.parameter_types)
    }

    /// Call host function `name` with `args`, as Hyperlight calls it for
    /// a guest: an unknown name, or arguments of the wrong number or types,
    /// is an error, before any handler runs.
    pub fn call(
        &self,
        name: &str,
        args: Vec<ParameterValue>,
    ) -> hyperlight_host::Result<ReturnValue> {
        let entry = self
            .functions
            .get(name)
            .ok_or_else(|| new_error!("no host function {name}"))?;
        (entry.call)(args)
    }

    /// The wait between two entries of the cooperative step model, which
    /// watches the sockets the guest's `net_poll` calls said it waits on:
    /// whether one became ready within `timeout`.
    pub fn net_wait(&self, timeout: Duration) -> bool {
        self.net.wait_ready(Some(timeout))
    }

    /// Drop every host socket, as an in-place restore does.
    pub fn net_reset(&self) {
        self.net.reset();
    }
}

/// The names a DNS query asks about, as the policy reads a guest's query
/// to decide whether to send it.
pub fn dns_question_names(data: &[u8]) -> Option<Vec<String>> {
    net_policy::dns_question_names(data)
}

/// The name a DNS answer is for and the addresses it gives, as the policy
/// reads an answer from a resolver.
pub fn dns_answer_ips(data: &[u8]) -> Option<(String, Vec<IpAddr>)> {
    net_policy::dns_answer_ips(data)
}

/// Whether `policy` lets the guest send the DNS query `data`.
pub fn allows_query(policy: &NetworkPolicy, data: &[u8]) -> bool {
    policy.allows_query(data)
}

/// Have `policy` learn from the DNS answer `data` that came from `from`.
pub fn learn_from_dns_answer(policy: &NetworkPolicy, from: SocketAddr, data: &[u8]) {
    policy.learn_from_dns_answer(from, data);
}

/// Whether `policy` lets the guest reach `addr`, on a UDP socket or not.
pub fn allows(policy: &NetworkPolicy, addr: &SocketAddr, udp: bool) -> bool {
    policy.allows(addr, udp)
}
