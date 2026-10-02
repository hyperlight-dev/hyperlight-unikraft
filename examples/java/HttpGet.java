// HTTP GET over host networking.  Run with: --net
import java.net.http.*;

void main() throws Exception {
    try (var client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build()) {
        var request = HttpRequest.newBuilder(URI.create("http://httpbin.org/get"))
                .timeout(Duration.ofSeconds(10))
                .build();
        var response = client.send(request, HttpResponse.BodyHandlers.ofString());
        IO.println("Status: " + response.statusCode());
    }
}
