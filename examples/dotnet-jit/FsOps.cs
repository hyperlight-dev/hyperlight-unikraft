// Filesystem I/O against a host-mounted directory.
// Run with: --mount <hostdir>:/mnt/host
var path = "/mnt/host/dotnet_fs.txt";

File.WriteAllText(path, "hello from dotnet");
Console.WriteLine($"read-back: {File.ReadAllText(path)}");

File.WriteAllLines("/mnt/host/dotnet_lines.txt", new[] { "a", "b", "c" });
Console.WriteLine($"line-count: {File.ReadAllLines("/mnt/host/dotnet_lines.txt").Length}");

Console.WriteLine($"exists: {File.Exists(path)}");
Console.WriteLine("fs-ops-done");
