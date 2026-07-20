'use strict';

// Keepalive timer: prevents the Node.js event loop from exiting and
// gives the cooperative scheduler a periodic wakeup via epoll_wait timeout.
setInterval(() => {}, 100);

// Block child_process methods.  Unikraft's fork() is vfork() (shared
// address space); a child's V8 init corrupts the parent's heap and
// the subsequent abort leaves the server in an unrecoverable state.
// The extension host already runs as a web worker, so no functionality
// is lost.
const cp = require('child_process');
for (const fn of ['fork', 'spawn', 'exec', 'execFile',
                   'execSync', 'execFileSync', 'spawnSync']) {
  if (typeof cp[fn] === 'function') {
    cp[fn] = function () {
      const err = new Error(`[Hyperlight] child_process.${fn} disabled`);
      err.code = 'ENOSYS';
      throw err;
    };
  }
}

// Neutralise process.abort() — same vfork corruption risk.
process.abort = () => {
  console.error('[Hyperlight] process.abort() suppressed');
};

// Block outbound HTTPS/TLS connections.  On single-vCPU, TLS handshakes
// starve the event loop and prevent IPC (scanExtensions, etc.) from being
// processed.  The browser downloads VSIXs directly from the CDN via CORS,
// so the server doesn't need outbound network access.
const tls = require('tls');
const _tlsConnect = tls.connect;
tls.connect = function (...args) {
  const opts = typeof args[0] === 'object' ? args[0] : { port: args[0], host: args[1] };
  const host = opts.host || opts.servername || 'unknown';
  // Allow connections to localhost (for internal IPC)
  if (host === 'localhost' || host === '127.0.0.1' || host === '::1') {
    return _tlsConnect.apply(this, args);
  }
  console.log('[Hyperlight] blocked TLS connect to:', host);
  const { Duplex } = require('stream');
  const fake = new Duplex({ read() {}, write(c, e, cb) { cb(); } });
  process.nextTick(() => fake.destroy(Object.assign(new Error('ECONNREFUSED'), { code: 'ECONNREFUSED' })));
  return fake;
};

// Catch otherwise-fatal exceptions so the server stays alive.
process.on('uncaughtException', (err) => {
  console.error('[Hyperlight] uncaughtException (non-fatal):', err.message);
});
