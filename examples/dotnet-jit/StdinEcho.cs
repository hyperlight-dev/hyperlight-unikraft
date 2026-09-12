// Read from stdin and echo each line.
var lines = new List<string>();
string? line;
while ((line = Console.ReadLine()) != null)
{
    lines.Add(line);
}
Console.WriteLine($"lines={lines.Count}");
foreach (var l in lines)
{
    Console.WriteLine($"echo: {l}");
}
Console.WriteLine("stdin-done");
