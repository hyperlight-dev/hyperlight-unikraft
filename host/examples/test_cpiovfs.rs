use hyperlight_unikraft::Sandbox;

fn main() -> anyhow::Result<()> {
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
        let _: () = sbox.call_named("init", ())?;
        eprintln!("  init OK");
        sbox.snapshot_now()?;
        eprintln!("  snapshot OK");

        let snap_path = "/tmp/cpiovfs_snapshot.hls";
        sbox.save_snapshot(snap_path)?;
        let snap_size = std::fs::metadata(snap_path)?.len();
        eprintln!(
            "  snapshot size: {} MiB ({} bytes)",
            snap_size / 1024 / 1024,
            snap_size
        );

        sbox.restore()?;
        eprintln!("  restore OK");
        let _: () = sbox.call_named("run", "print('test ok')".to_string())?;
        eprintln!("  call OK");
    }

    eprintln!("=== All tests passed ===");
    Ok(())
}
