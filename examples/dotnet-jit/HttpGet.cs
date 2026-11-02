// HTTP GET over host networking.  Run with: --net
// Generous: the GET runs well under a second interactively, but the nightly's
// nested-virt arm64 runner is slow enough to blow past 10 s (#241).
using var client = new System.Net.Http.HttpClient { Timeout = TimeSpan.FromSeconds(60) };
var resp = await client.GetAsync("http://httpbin.org/get");
Console.WriteLine($"Status: {(int)resp.StatusCode}");
