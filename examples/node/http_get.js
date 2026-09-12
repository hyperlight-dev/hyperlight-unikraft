// HTTP GET — demonstrates host networking.

const http = require('http');

const url = 'http://httpbin.org/get';
console.log(`GET ${url}`);

http.get(url, { timeout: 10000 }, (res) => {
    let body = '';
    res.on('data', chunk => body += chunk);
    res.on('end', () => {
        console.log(`Status: ${res.statusCode}`);
        console.log(`Content-Type: ${res.headers['content-type']}`);
        console.log(body.slice(0, 500));
    });
}).on('error', (e) => {
    console.error(`Request failed: ${e.message}`);
});
