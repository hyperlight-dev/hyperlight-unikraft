// A Kestrel server inside a Hyperlight micro-VM.
//
// Binds 0.0.0.0:8080; the manifest's [run.net] enables networking and
// allows the port, so the host reaches it at http://127.0.0.1:8080.
var builder = WebApplication.CreateEmptyBuilder(new WebApplicationOptions());
builder.WebHost.UseKestrelCore();

var app = builder.Build();
app.Urls.Add("http://0.0.0.0:8080");

app.Run(async context =>
{
    await context.Response.WriteAsync("Hello from {{name}} on Kestrel, inside a Hyperlight micro-VM!\n");
});

Console.WriteLine("Listening on http://127.0.0.1:8080");
await app.RunAsync();
