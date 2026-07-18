// Pre-fetch TextMate tokenization resources while the server is alive.
// Runs as a module with top-level await BEFORE workbench.js, blocking its
// evaluation until all fetches complete.  After the extension host (PID 15)
// starts and starves the single-vCPU server, the monkey-patched APIs serve
// these cached responses instead of hitting the network.

const base = (globalThis._VSCODE_FILE_ROOT || '').replace(/\/out\/$/, '');

const themeNames = [
  '2026-light.json', '2026-dark.json', 'dark_plus.json', 'light_plus.json',
  'dark_modern.json', 'light_modern.json', 'dark_vs.json', 'light_vs.json',
  'hc_black.json', 'hc_light.json',
];

const [textmateJs, onigurumaJs, wasmBuf,
       tomlGrammar, tomlConfig, tomlFm, tomlMd,
       tmWorker,
       ...themeTexts] =
  await Promise.all([
    fetch(base + '/node_modules.asar/vscode-textmate/release/main.js').then(r => r.text()),
    fetch(base + '/node_modules.asar/vscode-oniguruma/release/main.js').then(r => r.text()),
    fetch(base + '/node_modules.asar.unpacked/vscode-oniguruma/release/onig.wasm').then(r => r.arrayBuffer()),
    fetch(base + '/extensions/toml/toml.tmLanguage.json').then(r => r.text()),
    fetch(base + '/extensions/toml/language-configuration.json').then(r => r.text()),
    fetch(base + '/extensions/toml/toml.frontmatter.tmLanguage.json').then(r => r.text()),
    fetch(base + '/extensions/toml/toml.markdown.tmLanguage.json').then(r => r.text()),
    fetch(base + '/out/vs/workbench/services/textMate/browser/backgroundTokenization/worker/textMateTokenizationWorker.workerMain.js').then(r => r.text()),
    ...themeNames.map(n =>
      fetch(base + '/extensions/theme-defaults/themes/' + n).then(r => r.text())),
  ]);

const themeCache = {};
themeNames.forEach((n, i) => { themeCache[n] = themeTexts[i]; });

const blobUrls = {
  'vscode-textmate/release/main.js':
    URL.createObjectURL(new Blob([textmateJs], { type: 'text/javascript' })),
  'vscode-oniguruma/release/main.js':
    URL.createObjectURL(new Blob([onigurumaJs], { type: 'text/javascript' })),
};

// --- fetch() interception ---
// Eagerly-cached resources are served immediately.  Everything else uses
// a cache-on-success / serve-from-cache-on-failure strategy so that any
// resource fetched while the server was alive survives the starvation.
const responseCache = new Map();
const _fetch = window.fetch;

window.fetch = function (url, ...rest) {
  const s = String(typeof url === 'string' ? url : url?.url || '');

  // Pre-cached: Oniguruma WASM
  if (s.includes('onig.wasm'))
    return Promise.resolve(new Response(wasmBuf.slice(0)));

  // Pre-cached: TOML grammars & config
  if (s.includes('toml.tmLanguage.json') && !s.includes('frontmatter') && !s.includes('markdown'))
    return Promise.resolve(new Response(tomlGrammar, { headers: { 'content-type': 'application/json' } }));
  if (s.includes('toml.frontmatter.tmLanguage.json'))
    return Promise.resolve(new Response(tomlFm, { headers: { 'content-type': 'application/json' } }));
  if (s.includes('toml.markdown.tmLanguage.json'))
    return Promise.resolve(new Response(tomlMd, { headers: { 'content-type': 'application/json' } }));
  if (s.includes('language-configuration.json') && s.includes('toml'))
    return Promise.resolve(new Response(tomlConfig, { headers: { 'content-type': 'application/json' } }));

  // Pre-cached: theme files
  for (const [name, content] of Object.entries(themeCache)) {
    if (s.includes('/themes/' + name))
      return Promise.resolve(new Response(content, { headers: { 'content-type': 'application/json' } }));
  }

  // Fallback: cache-on-success, serve-from-cache-on-failure
  return _fetch.apply(this, [url, ...rest]).then(
    response => {
      if (response.ok && (s.includes('/extensions/') || s.includes('/node_modules')))
        responseCache.set(s, response.clone());
      return response;
    },
    error => {
      const cached = responseCache.get(s);
      if (cached) return cached.clone();
      throw error;
    }
  );
};

// --- AMD script loading (setAttribute interception) ---
const _setAttribute = HTMLScriptElement.prototype.setAttribute;
HTMLScriptElement.prototype.setAttribute = function (name, value) {
  if (name === 'src') {
    const v = String(value);
    for (const [pattern, blobUrl] of Object.entries(blobUrls)) {
      if (v.includes(pattern))
        return _setAttribute.call(this, 'src', blobUrl);
    }
  }
  return _setAttribute.apply(this, arguments);
};

// --- TextMate background worker inlining ---
// The workbench creates module Workers via blob URLs that contain:
//   await import(ttPolicy?.createScriptURL("workerUrl") ?? "workerUrl");
// When the server is dead, that import fails.  We intercept Blob
// construction and replace the dynamic import with the pre-fetched
// worker code so the worker is fully self-contained.
const _Blob = window.Blob;
window.Blob = function (parts, opts) {
  if (parts?.length === 1 && typeof parts[0] === 'string' &&
      parts[0].includes('textMateTokenizationWorker')) {
    const src = parts[0];
    const patched = src.replace(
      /await import\(ttPolicy\?\.createScriptURL\([^)]*\)\s*\?\?\s*[^)]*\)/,
      '\n' + tmWorker + '\n'
    );
    if (patched !== src)
      return new _Blob([patched], opts);
  }
  return new _Blob(parts, opts);
};
window.Blob.prototype = _Blob.prototype;
