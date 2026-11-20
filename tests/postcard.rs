// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

#![cfg(all(unix, any(target_arch = "x86_64", target_arch = "aarch64")))]

use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use hyperlight_common::flatbuffer_wrappers::{
    ExternalValueSource,
    function_call::{FunctionCall, FunctionCallType},
    function_types::{FunctionCallResult, ParameterValue, ReturnType, ReturnValue},
    guest_error::{ErrorCode, GuestError},
    guest_log_data::GuestLogData,
    guest_log_level::LogLevel,
};
use hyperlight_common::transport::{EncodedMessage, ExternalValues, MsgKind};

#[path = "../drivers/wasmtime/driver/src/call.rs"]
mod wasmtime_call;

struct Probe(tempfile::TempDir);

impl Probe {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let kernel = root.join("kernel/unikraft");
        let arch = if cfg!(target_arch = "aarch64") {
            "arm/arm64"
        } else {
            "x86/x86_64"
        };
        let output = Command::new("cc")
            .args(["-std=c11", "-O2", "-Wall", "-Wextra", "-Werror"])
            .arg("-I")
            .arg(kernel.join("include"))
            .arg("-I")
            .arg(kernel.join(format!("arch/{arch}/include")))
            .arg("-I")
            .arg(kernel.join("plat/hyperlight/include"))
            .arg(root.join("tests/fixtures/postcard.c"))
            .arg("-o")
            .arg(dir.path().join("probe"))
            .output()
            .expect("wire tests require a native C compiler");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Self(dir)
    }

    fn run(&self, mode: &str, control: usize, data: &[u8]) -> Output {
        let input = self.0.path().join("input");
        let mut bytes = u32::try_from(control).unwrap().to_le_bytes().to_vec();
        bytes.extend_from_slice(data);
        fs::write(&input, bytes).unwrap();
        Command::new(self.0.path().join("probe"))
            .arg(mode)
            .stdin(Stdio::from(fs::File::open(input).unwrap()))
            .output()
            .unwrap()
    }
}

struct Externals<'a>(&'a [u8]);

impl ExternalValueSource for Externals<'_> {
    fn take_bytes(&mut self, length: usize) -> anyhow::Result<Vec<u8>> {
        let (value, rest) = self
            .0
            .split_at_checked(length)
            .ok_or_else(|| anyhow::anyhow!("truncated external value"))?;
        self.0 = rest;
        Ok(value.to_vec())
    }
    fn take_chunks(&mut self, length: usize) -> anyhow::Result<Vec<bytes::Bytes>> {
        Ok(vec![self.take_bytes(length)?.into()])
    }
    fn finish(&mut self) -> anyhow::Result<()> {
        anyhow::ensure!(self.0.is_empty(), "unused external bytes");
        Ok(())
    }
}

fn payload(control: &[u8], external: ExternalValues<'_>) -> Vec<u8> {
    let message = EncodedMessage::new(MsgKind::Request, 1, control, external).unwrap();
    control
        .iter()
        .copied()
        .chain(message.external_chunks().flatten().copied())
        .collect()
}

#[test]
fn c_encoder_and_log_match_upstream_postcard() {
    let probe = Probe::new();
    let output = probe.run("encode", 0, &[]);
    assert!(output.status.success(), "{output:?}");
    let newline = output.stdout.iter().position(|b| *b == b'\n').unwrap();
    let control: usize = std::str::from_utf8(&output.stdout[..newline])
        .unwrap()
        .parse()
        .unwrap();
    let data = &output.stdout[newline + 1..];
    let call = FunctionCall::decode(&data[..control], &mut Externals(&data[control..])).unwrap();
    assert_eq!(call.function_name, "HostCall");
    assert_eq!(
        call.parameters.unwrap(),
        vec![
            ParameterValue::Int(i32::MIN),
            ParameterValue::UInt(u32::MAX),
            ParameterValue::Long(i64::MIN),
            ParameterValue::ULong(u64::MAX),
            ParameterValue::String("hello".into()),
            ParameterValue::VecBytes(b"first".to_vec()),
            ParameterValue::VecBytes(vec![]),
            ParameterValue::VecBytes(b"last".to_vec()),
        ]
    );
    let output = probe.run("log", 0, &[]);
    assert!(output.status.success());
    let log = GuestLogData::try_from(output.stdout.as_slice()).unwrap();
    assert_eq!(
        log,
        GuestLogData::new(
            "bad?".into(),
            "kernel".into(),
            LogLevel::Warning,
            "".into(),
            "test.c".into(),
            300
        )
    );
}

#[test]
fn kernel_and_runtime_decoders_accept_calls_and_reject_malformed_input() {
    let probe = Probe::new();
    for text in ["", "a".repeat(65536).as_str(), "hello \u{1f30d}"] {
        let call = FunctionCall::new(
            "Call".into(),
            Some(vec![
                ParameterValue::String(text.into()),
                ParameterValue::String("input".into()),
                ParameterValue::ULong(u64::MAX),
            ]),
            FunctionCallType::Guest,
            ReturnType::Void,
        );
        let mut buffer = vec![];
        let mut external = ExternalValues::new();
        let control = call.encode(&mut buffer, &mut external).unwrap();
        let data = payload(control, external);
        let output = probe.run("call", control.len(), &data);
        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout, text.as_bytes());
        let decoded = wasmtime_call::Call::parse(&data).unwrap();
        assert_eq!(decoded.name(), Some("Call"));
        assert_eq!(decoded.string_arg(0), Some(text));
        assert_eq!(decoded.string_arg(1), Some("input"));
        assert_eq!(decoded.string_arg(2), None);
        for end in [0, 1, data.len() - 1] {
            let output = probe.run("call", control.len(), &data[..end]);
            assert_eq!(output.status.code(), Some(1));
            assert!(wasmtime_call::Call::parse(&data[..end]).is_none());
            assert_eq!(
                probe.run("driver", end, &data[..end]).status.code(),
                Some(1)
            );
        }
        let mut trailing = data.clone();
        trailing.push(0);
        assert_eq!(
            probe.run("call", control.len(), &trailing).status.code(),
            Some(1)
        );
        assert_eq!(
            probe.run("driver", trailing.len(), &trailing).status.code(),
            Some(1)
        );
        assert!(wasmtime_call::Call::parse(&trailing).is_none());
    }
    for data in [
        vec![0x80; 12],
        vec![0, 2, 8, 0], // invalid host boolean
        vec![
            0, 0, 8, 1, 3, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 2,
        ],
        vec![0, 0, 8, 1, 7, 2],      // invalid parameter boolean
        vec![0, 0, 8, 1, 9, 127, 0], // missing external payload
    ] {
        assert_eq!(probe.run("call", data.len(), &data).status.code(), Some(1));
        assert_eq!(
            probe.run("driver", data.len(), &data).status.code(),
            Some(1)
        );
        assert!(wasmtime_call::Call::parse(&data).is_none());
    }
}

#[test]
fn c_result_decoder_checks_bounds_types_errors_and_external_lengths() {
    let probe = Probe::new();
    let cases = [
        (
            Ok(ReturnValue::Int(i32::MIN)),
            "0 0 4294967295 0\n".to_string(),
        ),
        (Ok(ReturnValue::Int(-1)), "0 0 1 0\n".into()),
        (
            Ok(ReturnValue::ULong(u64::MAX)),
            format!("0 3 {} 0\n", u64::MAX),
        ),
        (Ok(ReturnValue::Void(())), "0 8 0 0\n".into()),
        (
            Ok(ReturnValue::String("hello".into())),
            "0 6 0 5\nhello".into(),
        ),
        (
            Ok(ReturnValue::VecBytes(b"hello".to_vec())),
            "0 9 0 5\nhello".into(),
        ),
        (Ok(ReturnValue::VecBytes(vec![])), "0 9 0 0\n".into()),
        (
            Err(GuestError::new(
                ErrorCode::HostFunctionError,
                "failed".into(),
            )),
            "1 0 17 6\nfailed".into(),
        ),
    ];
    for (value, expected) in cases {
        let result = FunctionCallResult::new(value);
        let mut buffer = vec![];
        let mut external = ExternalValues::new();
        let control = result.encode(&mut buffer, &mut external).unwrap();
        let data = payload(control, external);
        let output = probe.run("result", control.len(), &data);
        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout, expected.as_bytes());
        for end in 0..data.len() {
            assert_eq!(
                probe
                    .run("result", control.len(), &data[..end])
                    .status
                    .code(),
                Some(1)
            );
        }
        let mut trailing = data.clone();
        trailing.push(0);
        assert_eq!(
            probe.run("result", control.len(), &trailing).status.code(),
            Some(1)
        );
    }
    for data in [
        vec![0, 0, 0xff, 0xff, 0xff, 0xff, 0x10], // overflowing u32
        vec![
            0, 3, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 2,
        ],
        vec![0, 7, 0x80, 0], // a bool is a single byte
        vec![0, 6, 1, 0xff], // invalid UTF-8
        vec![0, 9, 0, 2],    // invalid chunked boolean
        vec![0, 8, 0],       // trailing control byte
    ] {
        assert_eq!(
            probe.run("result", data.len(), &data).status.code(),
            Some(1)
        );
    }
}
