// An Express server inside a Hyperlight micro-VM.
//
// The guest binds 0.0.0.0:8080; hluk backs guest sockets with host sockets,
// so the listener is reachable from the host at http://127.0.0.1:8080. The
// manifest's [run.net] enables networking and allows the port.
const express = require('express');
const app = express();

app.get('/', (req, res) => {
  res.type('text/plain').send('Hello from {{name}} on Express, inside a Hyperlight micro-VM!\n');
});

app.get('/health', (req, res) => {
  res.json({ status: 'ok', app: '{{name}}', isolation: 'hyperlight-micro-vm' });
});

app.listen(8080, '0.0.0.0', () => {
  console.log('Listening on http://127.0.0.1:8080');
});
