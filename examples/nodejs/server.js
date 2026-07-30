// Minimal HTTP server, to exercise host-proxied sockets.
//
//   hyperlight-unikraft <kernel> --initrd node-initrd.cpio --memory 512Mi \
//       --poll --net --port 8080 -- /app/server.js
//
// --poll is required: without it the guest gets a single run-to-completion
// call and the accept loop never runs. --port allowlists the bind.

const http = require('http');

const port = Number(process.env.PORT || process.argv[2] || 8080);
let requests = 0;

const server = http.createServer((req, res) => {
  requests++;
  res.writeHead(200, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify({ message: 'hello from Node.js on Hyperlight', path: req.url, requests }) + '\n');
});

server.on('error', (err) => {
  console.error(`listen failed: ${err.code} ${err.message}`);
  process.exit(1);
});

server.listen(port, '0.0.0.0', () => {
  console.log(`listening on ${port}`);
});
