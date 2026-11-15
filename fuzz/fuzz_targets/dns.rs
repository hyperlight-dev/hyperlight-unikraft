// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The DNS messages the network policy reads: a guest's query, which
//! `net_send` and `net_sendto` check before it leaves for port 53, and a
//! resolver's answer, which `net_recvfrom` learns the addresses of names
//! from, under a policy that allows names rather than addresses.
//!
//! Each input is a message and where it came from: the policy's resolver,
//! the resolver on another port, or another host.  A name read from it
//! must be one a resolver could be asked about; the policy must allow a
//! query exactly when its filter allows every name the query asks; and an
//! answer must teach the addresses of a name the filter allows when it
//! comes from the resolver, and nothing otherwise, nor ever make loopback,
//! the unspecified address or a metadata address reachable.

#![no_main]

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use hyperlight_unikraft::fuzzing::{
    allows, allows_query, dns_answer_ips, dns_question_names, learn_from_dns_answer,
};
use hyperlight_unikraft::{Destination, Filter, NetworkFilter, NetworkPolicy};
use libfuzzer_sys::arbitrary::{self, Arbitrary};
use libfuzzer_sys::fuzz_target;

/// The resolver the policy lets the guest ask.
const RESOLVER: Ipv4Addr = Ipv4Addr::new(192, 0, 2, 53);
/// A host that is not one.
const STRANGER: Ipv4Addr = Ipv4Addr::new(192, 0, 2, 99);
/// AWS's instance metadata service, over IPv6.
const AWS_METADATA: Ipv6Addr = Ipv6Addr::new(0xfd00, 0xec2, 0, 0, 0, 0, 0, 0x254);
/// The longest name a resolver can be asked about, in its dotted form.
const MAX_NAME_LEN: usize = 253;

fuzz_target!(|input: Input| run(input));

#[derive(Arbitrary, Debug)]
struct Input {
    from: Sender,
    message: Message,
}

/// A message: any bytes, or one put together as DNS lays it out, of names
/// the filter allows and names it does not, compression pointers and
/// records of any type, so that well-formed ones are common too.
#[derive(Arbitrary, Debug)]
enum Message {
    Bytes(Vec<u8>),
    Built(Built),
}

#[derive(Arbitrary, Debug)]
struct Built {
    response: bool,
    opcode: u8,
    questions: Vec<WireName>,
    answers: Vec<Record>,
    /// The section counts the header gives, when not the sections' own.
    counts: Option<(u16, u16)>,
    trailer: Vec<u8>,
}

#[derive(Arbitrary, Debug)]
struct Record {
    name: WireName,
    data: RecordData,
}

#[derive(Arbitrary, Debug)]
enum RecordData {
    A([u8; 4]),
    Aaaa([u8; 16]),
    Other(u16, Vec<u8>),
}

#[derive(Arbitrary, Debug)]
struct WireName {
    labels: Vec<Label>,
    /// Ends at the root, or with a pointer to this offset (12 is the
    /// first question's name).
    pointer: Option<u16>,
}

#[derive(Arbitrary, Debug)]
enum Label {
    Known(u8),
    Text(Vec<u8>),
    /// A byte where a label's length goes: a pointer's, an extended
    /// label type's, or a length past the message.
    Byte(u8),
}

/// The labels of `allowed.test`, other spellings, and others.
const LABELS: &[&str] = &[
    "allowed",
    "test",
    "ALLOWED",
    "Test",
    "x",
    "allowed.test",
    "",
];

impl Message {
    fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Bytes(bytes) => bytes.clone(),
            Self::Built(built) => built.bytes(),
        }
    }
}

impl Built {
    fn bytes(&self) -> Vec<u8> {
        let b = self;
        let (qdcount, ancount) = b.counts.unwrap_or((
            b.questions.len().try_into().unwrap_or(u16::MAX),
            b.answers.len().try_into().unwrap_or(u16::MAX),
        ));
        let flags = (u16::from(b.response) << 15) | (u16::from(b.opcode & 0xf) << 11);
        let mut m = vec![0x12, 0x34];
        m.extend(flags.to_be_bytes());
        m.extend(qdcount.to_be_bytes());
        m.extend(ancount.to_be_bytes());
        m.extend([0, 0, 0, 0]);
        for q in &b.questions {
            q.write(&mut m);
            m.extend([0, 1, 0, 1]);
        }
        for r in &b.answers {
            r.name.write(&mut m);
            let (rtype, data): (u16, &[u8]) = match &r.data {
                RecordData::A(a) => (1, a),
                RecordData::Aaaa(a) => (28, a),
                RecordData::Other(t, d) => (*t, d),
            };
            m.extend(rtype.to_be_bytes());
            m.extend([0, 1, 0, 0, 0, 60]);
            m.extend(u16::try_from(data.len()).unwrap_or(u16::MAX).to_be_bytes());
            m.extend(data);
        }
        m.extend(&b.trailer);
        m
    }
}

impl WireName {
    fn write(&self, m: &mut Vec<u8>) {
        for label in &self.labels {
            match label {
                Label::Known(i) => {
                    let text = LABELS[usize::from(*i) % LABELS.len()].as_bytes();
                    m.push(text.len() as u8);
                    m.extend(text);
                }
                Label::Text(text) => {
                    let text = &text[..text.len().min(63)];
                    m.push(text.len() as u8);
                    m.extend(text);
                }
                Label::Byte(b) => m.push(*b),
            }
        }
        match self.pointer {
            Some(offset) => m.extend((0xc000 | (offset & 0x3fff)).to_be_bytes()),
            None => m.push(0),
        }
    }
}

#[derive(Arbitrary, Debug, Clone, Copy, PartialEq)]
enum Sender {
    Resolver,
    ResolverOtherPort,
    Stranger,
}

impl Sender {
    fn addr(self) -> SocketAddr {
        match self {
            Self::Resolver => (RESOLVER, 53).into(),
            Self::ResolverOtherPort => (RESOLVER, 5353).into(),
            Self::Stranger => (STRANGER, 53).into(),
        }
    }
}

/// Allows the names under `allowed.test`, and an address DNS said one of
/// them has.
struct Names;

fn allowed(name: &str) -> bool {
    name == "allowed.test" || name.ends_with(".allowed.test")
}

impl NetworkFilter for Names {
    fn allows_name(&self, name: &str) -> bool {
        allowed(name)
    }

    fn allows(&self, destination: &Destination<'_>) -> bool {
        !destination.names.is_empty()
    }
}

/// A name as DNS carries it: at most 253 characters, in labels of printable
/// ASCII with no dot, or the root's empty name.
fn check_name(name: &str) {
    assert!(
        name.len() <= MAX_NAME_LEN,
        "a {}-character name",
        name.len()
    );
    assert!(
        name.is_empty()
            || name
                .split('.')
                .all(|l| !l.is_empty() && l.bytes().all(|b| b.is_ascii_graphic())),
        "the name {name:?}"
    );
}

fn run(input: Input) {
    let filter = Filter::new(Names);
    filter.exempt_resolvers([IpAddr::V4(RESOLVER)]);
    let policy = NetworkPolicy::Filter(filter);
    let message = input.message.bytes();
    let data = &message;

    let questions = dns_question_names(data);
    for name in questions.iter().flatten() {
        check_name(name);
    }
    let expected =
        questions.is_some_and(|names| names.iter().all(|n| allowed(&n.to_ascii_lowercase())));
    assert_eq!(
        allows_query(&policy, data),
        expected,
        "a query for {:?}",
        dns_question_names(data)
    );

    let answer = dns_answer_ips(data);
    learn_from_dns_answer(&policy, input.from.addr(), data);
    let Some((name, ips)) = answer else {
        return;
    };
    check_name(&name);
    let taught = input.from == Sender::Resolver && allowed(&name.to_ascii_lowercase());
    for ip in ips {
        let addr = SocketAddr::new(ip, 443);
        let reachable = allows(&policy, &addr, false);
        if is_local_or_metadata(ip) {
            assert!(!reachable, "an answer for {name:?} made {ip} reachable");
        } else {
            assert_eq!(
                reachable, taught,
                "an answer for {name:?} from {:?}: {ip}",
                input.from
            );
        }
    }
}

/// Loopback, unspecified or link-local, IPv4-mapped included: what no
/// policy but allowing everything lets a guest reach.
fn is_local_or_metadata(ip: IpAddr) -> bool {
    let ip = match ip {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or(ip, IpAddr::V4),
        v4 => v4,
    };
    match ip {
        IpAddr::V4(v4) => v4.is_loopback() || v4.is_unspecified() || v4.is_link_local(),
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unspecified()
                || (v6.segments()[0] & 0xffc0) == 0xfe80
                || v6 == AWS_METADATA
        }
    }
}
