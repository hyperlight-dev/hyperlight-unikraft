// HTTP GET over host networking.  Run with: --net
import java.net.http.*;

void main() throws Exception {
    // Generous timeouts: the GET runs well under a second interactively, but the
    // nightly's nested-virt arm64 runner is slow enough to blow past 10 s (#241).
    try (var client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(60)).build()) {
        var request = HttpRequest.newBuilder(URI.create("http://httpbin.org/get"))
                .timeout(Duration.ofSeconds(60))
                .build();
        var response = client.send(request, HttpResponse.BodyHandlers.ofString());
        IO.println("Status: " + response.statusCode());
    }
}
