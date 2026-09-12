// Express HTTP server running inside a Hyperlight micro-VM.
//
// The guest binds 0.0.0.0:8080; hluk backs guest sockets with host sockets,
// so the listener is reachable from the host at http://127.0.0.1:8080.
// Run with `--net --port 8080` so the socket layer is registered and the
// guest is allowed to bind that port.
const express = require('express');
const app = express();

app.get('/', (req, res) => {
  res.type('text/plain').send('Hello from Express on Hyperlight!\n');
});

app.get('/health', (req, res) => {
  res.json({ status: 'ok', server: 'express', isolation: 'hyperlight-micro-vm' });
});

app.listen(8080, '0.0.0.0', () => {
  console.log('Listening on :8080');
});
