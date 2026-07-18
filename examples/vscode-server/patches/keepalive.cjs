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

// Catch otherwise-fatal exceptions so the server stays alive.
process.on('uncaughtException', (err) => {
  console.error('[Hyperlight] uncaughtException (non-fatal):', err.message);
});
