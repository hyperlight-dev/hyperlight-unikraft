// Read from stdin and echo each line.
void main() throws IOException {
    var in = new BufferedReader(new InputStreamReader(System.in));
    var lines = new ArrayList<String>();
    String line;
    while ((line = in.readLine()) != null)
        lines.add(line);
    IO.println("lines=" + lines.size());
    for (var l : lines)
        IO.println("echo: " + l);
    IO.println("stdin-done");
}
