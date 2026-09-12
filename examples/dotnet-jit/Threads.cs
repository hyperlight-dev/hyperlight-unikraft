// Concurrency: Tasks (thread pool) and explicit Threads with a lock.
var tasks = new List<Task<int>>();
for (int i = 0; i < 8; i++)
{
    int n = i;
    tasks.Add(Task.Run(() => n * n));
}
int sum = 0;
foreach (var t in tasks)
    sum += t.Result;
Console.WriteLine($"sum-of-squares: {sum}"); // 0+1+4+9+16+25+36+49 = 140

int counter = 0;
var gate = new object();
var threads = new List<Thread>();
for (int i = 0; i < 4; i++)
{
    var th = new Thread(() => { lock (gate) counter++; });
    threads.Add(th);
    th.Start();
}
foreach (var th in threads)
    th.Join();
Console.WriteLine($"thread-counter: {counter}");
Console.WriteLine("threads-done");
