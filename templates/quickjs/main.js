// A script: what it declares stays for the next call, so a handler defined
// here (or in warm_exec, to have it in the snapshot) can be called many
// times from an embedder with AppSandbox::call("handler", json).
function handler(event) {
  return { greeting: `Hello, ${event.name}!` };
}

console.log(`Hello from {{name}}!`);
console.log(JSON.stringify(handler({ name: "QuickJS on Hyperlight" })));
