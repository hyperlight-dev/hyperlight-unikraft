var myVar = Environment.GetEnvironmentVariable("MY_VAR") ?? "";
var debug = Environment.GetEnvironmentVariable("DEBUG") ?? "";
var greeting = Environment.GetEnvironmentVariable("GREETING") ?? "";
Console.WriteLine($"MY_VAR={myVar}");
Console.WriteLine($"DEBUG={debug}");
Console.WriteLine($"GREETING={greeting}");
