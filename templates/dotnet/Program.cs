// Top-level statements: this file is compiled inside the guest by Roslyn.
Console.WriteLine("Hello from {{name}}!");
Console.WriteLine($".NET {Environment.Version} on {System.Runtime.InteropServices.RuntimeInformation.OSDescription}, " +
                  "inside a Hyperlight micro-VM");
