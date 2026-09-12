// .NET AOT capability check: guest filesystem (ramfs) + host filesystem
// (a --mount dir) + concurrency, run as a prebuilt native executable mounted
// into the guest (see tests/compiled.rs).

// Guest filesystem: /tmp is in-guest ramfs, no host involved.
var guestPath = "/tmp/aot_guest_fs.txt";
File.WriteAllText(guestPath, "aot-guest-fs-data");
Console.WriteLine($"guest-fs-read-back: {File.ReadAllText(guestPath)}");

// Host filesystem: /mnt/out is a host-backed mount, so the test can verify
// the bytes landed on the host side.
var hostPath = "/mnt/out/aot_host_fs.txt";
File.WriteAllText(hostPath, "aot-host-fs-data");
Console.WriteLine($"host-fs-read-back: {File.ReadAllText(hostPath)}");

// Concurrency.
var tasks = new List<Task<int>>();
for (int i = 0; i < 8; i++)
{
    int n = i;
    tasks.Add(Task.Run(() => n * n));
}
int sum = 0;
foreach (var t in tasks)
    sum += t.Result;
Console.WriteLine($"sum-of-squares: {sum}"); // 140

Console.WriteLine("aot-caps-done");
