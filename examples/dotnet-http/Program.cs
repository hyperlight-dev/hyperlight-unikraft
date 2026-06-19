var builder = WebApplication.CreateEmptyBuilder(new WebApplicationOptions());
builder.WebHost.UseKestrelCore();

var app = builder.Build();
app.Urls.Add("http://0.0.0.0:8080");

app.Run(async context =>
{
    await context.Response.WriteAsync("Hello from Kestrel on Hyperlight!");
});

Console.WriteLine("Listening on :8080");
await app.RunAsync();
