// Minimal ASP.NET Core Kestrel HTTP server for Hyperlight.
//
// Binds 0.0.0.0:8080; run under hluk with `--net --port 8080` so the host can
// reach it at http://127.0.0.1:8080.
var builder = WebApplication.CreateEmptyBuilder(new WebApplicationOptions());
builder.WebHost.UseKestrelCore();

var app = builder.Build();
app.Urls.Add("http://0.0.0.0:8080");

app.Run(async context =>
{
    await context.Response.WriteAsync("Hello from Kestrel on Hyperlight!\n");
});

Console.WriteLine("Listening on :8080");
await app.RunAsync();
