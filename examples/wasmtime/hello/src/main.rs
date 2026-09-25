//! A WASI program for the wasmtime image: its arguments, an environment
//! variable, and a file in a mounted directory.  Built for wasm32-wasip1 it
//! is a core module, for wasm32-wasip2 a component; the image runs both.
//!
//!     cargo build --release --target wasm32-wasip2
//!     hluk run --runtime wasmtime --mount ./data:/mnt/data \
//!         --guest-exec "/mnt/data/hello.wasm world"

use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let who = args.get(1).map_or("Hyperlight", String::as_str);
    println!("Hello, {who}, from WebAssembly on Hyperlight!");
    if let Ok(greeting) = env::var("GREETING") {
        println!("GREETING={greeting}");
    }
    if let Ok(text) = fs::read_to_string("/mnt/data/input.txt") {
        let words = text.split_whitespace().count();
        println!("input.txt has {words} words");
        fs::write("/mnt/data/output.txt", format!("{words}\n")).expect("write output.txt");
    }
    if who == "fail" {
        process::exit(3);
    }
}
