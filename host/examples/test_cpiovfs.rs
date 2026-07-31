use hyperlight_unikraft::Sandbox;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let kernel = std::env::args()
        .nth(1)
        .expect("usage: test_cpiovfs <kernel> <initrd>");
    let initrd = std::env::args()
        .nth(2)
        .expect("usage: test_cpiovfs <kernel> <initrd>");

    eprintln!("=== Test: build + init + snapshot + restore + call ===");
    {
        let mut sbox = Sandbox::builder(&kernel)
            .initrd_file(&initrd)
            .heap_size(3 * 512 * 1024 * 1024)
            .build()?;
        eprintln!("  build OK");
        sbox.restore()?;
        sbox.call_named_async("init", ()).await?;
        eprintln!("  init OK");
        sbox.snapshot_now()?;
        eprintln!("  snapshot OK");

        let snap_path = "/tmp/cpiovfs_snapshot";
        sbox.save_snapshot(snap_path)?;
        let snap_size: u64 = std::fs::read_dir(snap_path)?
            .filter_map(|e| e.ok())
            .filter_map(|e| e.metadata().ok().map(|m| m.len()))
            .sum();
        eprintln!(
            "  snapshot size: {} MiB ({} bytes)",
            snap_size / 1024 / 1024,
            snap_size
        );

        sbox.restore()?;
        eprintln!("  restore OK");
        sbox.call_named_async("run", "print('test ok')".to_string())
            .await?;
        eprintln!("  call OK");
    }

    eprintln!("=== All tests passed ===");
    Ok(())
}
