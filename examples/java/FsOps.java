// Filesystem I/O against a host-mounted directory.
// Run with: --mount <hostdir>:/mnt/host
void main() throws IOException {
    Path path = Path.of("/mnt/host/java_fs.txt");

    Files.writeString(path, "hello from java");
    IO.println("read-back: " + Files.readString(path));

    Path lines = Path.of("/mnt/host/java_lines.txt");
    Files.write(lines, List.of("a", "b", "c"));
    IO.println("line-count: " + Files.readAllLines(lines).size());

    IO.println("exists: " + Files.exists(path));
    IO.println("fs-ops-done");
}
