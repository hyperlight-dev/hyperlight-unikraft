"""Patch workbench.js for single-vCPU cooperative scheduling.

Handles both older (Sbo/wM/xM) and newer (cIo/AM/OM) VS Code versions.
"""
import re
import sys

f = "/rootfs/opt/vscode-server/out/vs/code/browser/workbench/workbench.js"
d = open(f).read()
count = 0

# 1. Gallery common headers: skip machine-ID lookup via IPC.
#    The original function conditionally calls xM/OM (readFile via IPC)
#    which hangs on single-vCPU.  Replace with a version that just
#    returns the two basic headers.
pattern = re.compile(
    r'async function (\w+)\(a,o,e,t,i,n,r\)\{let s=\{'
    r'"X-Market-Client-Id":`VSCode \$\{a\}`,'
    r'"User-Agent":`VSCode \$\{a\} \(\$\{o\.nameShort\}\)`\};'
    r'if\(\w+\(o,e\)&&\w+\(t\)===3\)\{let l=await \w+\(e,i,n\);'
    r's\["X-Market-User-Id"\]=l,s\["VSCode-SessionId"\]=r\.machineId\|\|l\}'
    r'return s\}'
)
m = pattern.search(d)
if m:
    fname = m.group(1)
    old = m.group(0)
    new = (
        f'async function {fname}(a,o)'
        '{return{"X-Market-Client-Id":`VSCode ${a}`,'
        '"User-Agent":`VSCode ${a} (${o.nameShort})`}}'
    )
    d = d.replace(old, new)
    print(f"Patched {fname} (gallery common headers)")
    count += 1
else:
    print("WARNING: gallery common headers pattern not found", file=sys.stderr)

# 2. WebSocket heartbeat timeout 20s -> 600s (browser-side)
#    Three sed-style replacements, same as the original Dockerfile sed.
replacements = [
    (r'e>=2e4&&t>=2e4&&i>=2e4&&!this._loadEstimator',
     r'e>=6e5&&t>=6e5&&i>=6e5&&!this._loadEstimator'),
    (r'e>=2e4&&t>=2e4&&!this._loadEstimator',
     r'e>=6e5&&t>=6e5&&!this._loadEstimator'),
    (r'Math.max(2e4-e,2e4-t,2e4-i,500)',
     r'Math.max(6e5-e,6e5-t,6e5-i,500)'),
]
for old_pat, new_pat in replacements:
    if old_pat in d:
        d = d.replace(old_pat, new_pat)
        count += 1

if count >= 4:
    print("Patched WebSocket heartbeat timeouts")
else:
    print("WARNING: some heartbeat patterns not found", file=sys.stderr)

# 2b. Bypass getExtensionsControlManifest entirely (hangs on single-vCPU).
#     The control manifest fetches from vscode-cdn.net through the WebSocket proxy,
#     which blocks the single vCPU.
#     Patch the gallery service implementation to return immediately:
old_ecm_impl = 'async getExtensionsControlManifest(){if(!await'
new_ecm_impl = 'async getExtensionsControlManifest(){return{malicious:[],deprecated:{},search:[],autoUpdate:{}}}async _ecm_disabled(){if(!await'
if old_ecm_impl in d:
    d = d.replace(old_ecm_impl, new_ecm_impl, 1)
    print("Patched getExtensionsControlManifest gallery impl bypass")
    count += 1
else:
    print("WARNING: getExtensionsControlManifest impl pattern not found", file=sys.stderr)

#     Also replace all extensionManagementService calls (IPC proxy path):
ecm_call = 'this.extensionManagementService.getExtensionsControlManifest()'
ecm_repl = 'Promise.resolve({malicious:[],deprecated:{},search:[],autoUpdate:{}})'
ecm_n = d.count(ecm_call)
if ecm_n > 0:
    d = d.replace(ecm_call, ecm_repl)
    print(f"Patched {ecm_n} extensionManagementService.getExtensionsControlManifest calls")
    count += 1
else:
    print("WARNING: extensionManagementService.getExtensionsControlManifest calls not found", file=sys.stderr)

# 3. Bypass remoteExtensions.canInstall IPC (hangs on single-vCPU).
#    getTargetPlatform() is proxied via channel.call("getTargetPlatform")
#    inside the J7 base class canInstall → isExtensionPlatformCompatible.
ci_old = 'canInstall(e){return this.server.extensionManagementService.canInstall(e)}'
ci_new = 'canInstall(e){return Promise.resolve(!0)}'
if ci_old in d:
    d = d.replace(ci_old, ci_new, 1)
    print("Patched remoteExtensions.canInstall bypass")
    count += 1
else:
    print("WARNING: remoteExtensions.canInstall pattern not found", file=sys.stderr)

# 4. Non-blocking remote extension scan: don't block UI on slow IPC.
#    On single-vCPU, the remote scanExtensions IPC hangs because PID 15
#    (extension host) starves PID 1. Add a timeout so web+builtin extension
#    contributions (languages, grammars) register even when the IPC is slow.
rscan_old = 'let[t,i]=await Promise.all([this._scanWebExtensions(),this._remoteExtensionsScannerService.scanExtensions()]);i.length&&e.emitOne(new wft(i)),e.emitOne(new Ift(t))'
rscan_new = (
    'let t=await this._scanWebExtensions();'
    'let i=[];'
    'try{i=await Promise.race(['
    'this._remoteExtensionsScannerService.scanExtensions(),'
    'new Promise(r=>setTimeout(()=>r([]),15000))'
    '])}catch(x){console.warn("Remote ext scan failed:",x)}'
    'i.length&&e.emitOne(new wft(i)),e.emitOne(new Ift(t))'
)

# 4b. (removed — TOML extension no longer pre-bundled)
if rscan_old in d:
    d = d.replace(rscan_old, rscan_new, 1)
    print("Patched remote extension scan timeout")
    count += 1
else:
    print("WARNING: remote extension scan timeout pattern not found", file=sys.stderr)

# 5. Browser-side VSIX download + server-side local install.
#    The marketplace CDN allows CORS (Access-Control-Allow-Origin: *), so
#    the browser can download the VSIX directly.  This avoids the server
#    having to fetch from the CDN while CPU-starved by the extension host.
#    The browser downloads the VSIX, base64-encodes it, and sends via IPC
#    to the server's installFromBuffer handler which writes to a temp file
#    and installs locally (fast, no network needed).
ifg_old = 'async installFromGallery(e,t,i){let n=await this.extensionGalleryService.getManifest'
ifg_new = (
    'async installFromGallery(e,t,i){'
    'let s=this.extensionManagementServerService.remoteExtensionManagementServer'
    '||this.extensionManagementServerService.localExtensionManagementServer;'
    'if(!s)throw new Error("No server");'
    'let ch=s.extensionManagementService.channel;'
    'let url=e.assets?.download?.uri;'
    'if(url&&ch){'
    'try{'
    'await Promise.race(['
    'ch.call("getTargetPlatform"),'
    'new Promise((_,r)=>setTimeout(()=>r(new Error("IPC timeout")),10000))'
    ']);'
    'console.log("[install] downloading VSIX:",url.substring(0,80));'
    'let resp=await fetch(url);'
    'let buf=await resp.arrayBuffer();'
    'let bytes=new Uint8Array(buf);'
    'console.log("[install] downloaded",bytes.length,"bytes");'
    # Small VSIX (<256KB): send in one shot
    'if(bytes.length<262144){'
    'let chunks=[];'
    'for(let j=0;j<bytes.length;j+=8192)'
    'chunks.push(String.fromCharCode.apply(null,bytes.subarray(j,Math.min(j+8192,bytes.length))));'
    'let b64=btoa(chunks.join(""));'
    'let result=await ch.call("installFromBuffer",[b64]);'
    'console.log("[install] done (single)");'
    'return result'
    '}else{'
    # Large VSIX: chunked upload (32KB chunks = ~43KB base64 per IPC call)
    'console.log("[install] chunked upload,",bytes.length,"bytes...");'
    'await Promise.race([ch.call("installChunkStart",[]),new Promise((_,r)=>setTimeout(()=>r(new Error("chunkStart timeout")),15000))]);'
    'console.log("[install] chunk session started");'
    'const CHUNK=32768;'
    'let sent=0;'
    'for(let off=0;off<bytes.length;off+=CHUNK){'
    'let slice=bytes.subarray(off,Math.min(off+CHUNK,bytes.length));'
    'let c=[];for(let j=0;j<slice.length;j+=8192)'
    'c.push(String.fromCharCode.apply(null,slice.subarray(j,Math.min(j+8192,slice.length))));'
    'let b64=btoa(c.join(""));'
    'await Promise.race([ch.call("installChunkData",[b64]),new Promise((_,r)=>setTimeout(()=>r(new Error("chunk timeout")),15000))]);'
    'sent+=slice.length;'
    'if(sent%(CHUNK*10)<CHUNK)console.log("[install] sent",sent,"/",bytes.length);'
    '}'
    'console.log("[install] all chunks sent, finishing...");'
    'let result=await Promise.race(['
    'ch.call("installChunkEnd",[]),'
    'new Promise((_,r)=>setTimeout(()=>r(new Error("install timeout")),120000))'
    ']);'
    'console.log("[install] done (chunked)");'
    'return result'
    '}'
    '}catch(err){console.warn("[install] browser install failed:",err)}'
    '}'
    'return s.extensionManagementService.installFromGallery(e,t||{})'
    '}async _installFromGallery_orig(e,t,i){let n=await this.extensionGalleryService.getManifest'
)
if ifg_old in d:
    d = d.replace(ifg_old, ifg_new, 1)
    print("Patched installFromGallery bypass (skip CORS manifest fetch)")
    count += 1
else:
    print("WARNING: installFromGallery pattern not found", file=sys.stderr)

assert count >= 1, "No patches applied to workbench.js"
open(f, "w").write(d)
print(f"Total: {count} patches applied")
