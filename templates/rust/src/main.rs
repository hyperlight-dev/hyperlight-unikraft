fn main() {
    println!("Hello from {{name}}!");
    println!(
        "Rust on {}/{}, inside a Hyperlight micro-VM",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
}
