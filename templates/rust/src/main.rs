// The guest is Linux on the host's CPU, so a build for any other OS (a
// Windows .exe, a macOS binary) makes a program the guest cannot run.
// Refuse it here, before it looks like a successful build.
#[cfg(not(target_os = "linux"))]
compile_error!(
    "this program runs in a Linux guest, so it must be built for Linux: run `hluk build` on \
     Linux, or in WSL on Windows"
);

fn main() {
    println!("Hello from {{name}}!");
    println!(
        "Rust on {}/{}, inside a Hyperlight micro-VM",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
}
