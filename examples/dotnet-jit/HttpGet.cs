// HTTP GET over host networking.  Run with: --net
using var client = new System.Net.Http.HttpClient { Timeout = TimeSpan.FromSeconds(10) };
var resp = await client.GetAsync("http://httpbin.org/get");
Console.WriteLine($"Status: {(int)resp.StatusCode}");
