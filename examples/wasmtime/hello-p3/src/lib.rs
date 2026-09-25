//! A WASI 0.3 command for the wasmtime image: `run` is an async export,
//! stdout is a stream, and two tasks wait on the clock at once.
//!
//!     cargo build --release --target wasm32-wasip2
//!     hluk run --runtime wasmtime --mount ./target/wasm32-wasip2/release:/app:ro \
//!         --guest-exec "/app/hello_p3.wasm world"

use wasip3::cli::{environment, stdout};
use wasip3::clocks::monotonic_clock;

wasip3::cli::command::export!(Hello);

struct Hello;

impl wasip3::exports::cli::run::Guest for Hello {
    async fn run() -> Result<(), ()> {
        let who = environment::get_arguments()
            .into_iter()
            .nth(1)
            .unwrap_or_else(|| "Hyperlight".into());
        let (mut tx, rx) = wasip3::wit_stream::new();
        let (written, ()) = futures::join!(async { stdout::write_via_stream(rx).await }, async {
            tx.write_all(format!("Hello, {who}, from WASI 0.3!\n").into_bytes())
                .await;
            // Two sleeps at once: the run takes one tick of 20 ms, not two.
            let start = monotonic_clock::now();
            futures::join!(
                monotonic_clock::wait_for(20_000_000),
                monotonic_clock::wait_for(20_000_000)
            );
            let ms = (monotonic_clock::now() - start) / 1_000_000;
            tx.write_all(format!("slept twice concurrently in {ms} ms\n").into_bytes())
                .await;
            drop(tx);
        });
        written.map_err(|_| ())
    }
}
