fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("Hello from {{name}}!");
    println!("A WASI component run by Wasmtime inside a Hyperlight micro-VM; args: {args:?}");
}
