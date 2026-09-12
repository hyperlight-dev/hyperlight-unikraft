// Basic computation — demonstrates stateless C# scripting.

int Fibonacci(int n)
{
    if (n <= 1) return n;
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++)
    {
        int c = a + b;
        a = b;
        b = c;
    }
    return b;
}

Console.WriteLine("=== .NET Math Demo ===");

// Fibonacci
for (int i = 0; i <= 10; i++)
    Console.Write($"{Fibonacci(i)} ");
Console.WriteLine();

// LINQ
var squares = Enumerable.Range(1, 10).Select(x => x * x).ToList();
Console.WriteLine($"Squares: {string.Join(", ", squares)}");

// String manipulation
var words = "hello from dotnet on hyperlight";
var title = string.Join(" ", words.Split(' ').Select(w =>
    char.ToUpper(w[0]) + w[1..]));
Console.WriteLine($"Title case: {title}");

Console.WriteLine("Math demo done");
