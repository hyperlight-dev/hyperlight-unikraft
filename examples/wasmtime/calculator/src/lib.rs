//! A library component for the wasmtime image: it imports the interface
//! `my:app/math`, which the embedder provides (see
//! `examples/host_functions.rs`), and exports a function and an async one
//! (WASI 0.3) that waits on a timer first.
//!
//!     cargo build --release --target wasm32-wasip2

wit_bindgen::generate!({ world: "calculator", path: "wit" });

use my::app::math;
use wasip3::clocks::monotonic_clock;

struct Calculator;

impl Guest for Calculator {
    fn sum_of_squares(a: i32, b: i32) -> i32 {
        math::add(a * a, b * b)
    }

    async fn slow_sum_of_squares(a: i32, b: i32, delay_ms: u64) -> i32 {
        monotonic_clock::wait_for(delay_ms * 1_000_000).await;
        math::add(a * a, b * b)
    }
}

export!(Calculator);
