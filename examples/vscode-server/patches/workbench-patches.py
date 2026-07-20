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

# 4. Non-blocking remote extension scan with extension activation support.
#    On single-vCPU, the remote scanExtensions IPC can hang. Add a 60s timeout
#    so web+builtin contributions register even when IPC is slow. Mark remote
#    extensions as isUnderDevelopment to bypass enablement filtering, and emit
#    via wft (not yft) so they participate in the standard running-location
#    assignment and aren't removed by the post-scan cleanup.
rscan_old = 'let[t,i]=await Promise.all([this._scanWebExtensions(),this._remoteExtensionsScannerService.scanExtensions()]);i.length&&e.emitOne(new wft(i)),e.emitOne(new Ift(t))'
rscan_new = (
    'let t=await this._scanWebExtensions();'
    'let i=[];'
    'try{'
    'i=await Promise.race(['
    'this._remoteExtensionsScannerService.scanExtensions(),'
    'new Promise(r=>setTimeout(()=>r([]),60000))'
    '])'
    '}catch(x){}'
    'let decl=[];let remote=[];'
    'i.forEach(x=>{'
    'x.isUnderDevelopment=true;'
    'if(!x.identifier||!x.identifier._lower)x.identifier={id:x.id,uuid:x.uuid,value:x.id,_lower:x.id.toLowerCase()};'
    'let hasGrammar=x.contributes&&(x.contributes.grammars||x.contributes.languages||x.contributes.themes);'
    'if(hasGrammar){'
    'delete x.main;x.browser="./browser.js";'
    'x.extensionKind=["web"];'
    'if(x.extensionLocation&&x.extensionLocation.with)x.extensionLocation=x.extensionLocation.with({scheme:"extension"});'
    'decl.push(x)'
    '}else{remote.push(x)}'
    '});'
    'if(remote.length)e.emitOne(new wft(remote));'
    'e.emitOne(new Ift(t.concat(decl)))'
)

if rscan_old in d:
    d = d.replace(rscan_old, rscan_new, 1)
    print("Patched remote extension scan timeout")
    count += 1
else:
    print("WARNING: remote extension scan timeout pattern not found", file=sys.stderr)

# 5. Browser-side VSIX download via IPC proxy bypass.
#    The install flow routes through the aggregator's 3-arg installFromGallery,
#    which calls getManifest + checks, then delegates to the IPC proxy's
#    installFromGallery(e,t).  The proxy calls channel.call("installFromGallery")
#    which hangs on single-vCPU.  Replace the IPC proxy's installFromGallery
#    with browser-side VSIX download + chunked upload.
ifg_proxy_old = (
    'installFromGallery(e,t){'
    'return Promise.resolve(this.channel.call("installFromGallery",[e,t]))'
    '.then(i=>Xj(i,null))}'
)
ifg_proxy_new = (
    'async installFromGallery(e,t){'
    'let _url=e?.assets?.download?.uri;'
    'if(_url){'
    'try{'
    'console.log("[Hyperlight] Browser-side install:",e.identifier?.id);'
    'let _resp=await fetch(_url);'
    'if(!_resp.ok)throw new Error("Fetch "+_resp.status);'
    'let _buf=await _resp.arrayBuffer();'
    'let _bytes=new Uint8Array(_buf);'
    'await Promise.race(['
    'this.channel.call("installChunkStart",[]),'
    'new Promise((_,r)=>setTimeout(()=>r(new Error("timeout")),15000))'
    ']);'
    'const _CHUNK=524288;'
    'for(let _off=0;_off<_bytes.length;_off+=_CHUNK){'
    'let _sl=_bytes.subarray(_off,Math.min(_off+_CHUNK,_bytes.length));'
    'let _c=[];for(let _j=0;_j<_sl.length;_j+=8192)'
    '_c.push(String.fromCharCode.apply(null,_sl.subarray(_j,Math.min(_j+8192,_sl.length))));'
    'let _b64=btoa(_c.join(""));'
    'await Promise.race(['
    'this.channel.call("installChunkData",[_b64]),'
    'new Promise((_,r)=>setTimeout(()=>r(new Error("timeout")),15000))'
    ']);'
    '}'
    'let _r=await Promise.race(['
    'this.channel.call("installChunkEnd",[]),'
    'new Promise((_,r)=>setTimeout(()=>r(new Error("timeout")),30000))'
    ']);'
    'let _local={identifier:e.identifier,type:1,isBuiltin:false,'
    'manifest:_r.manifest||{name:e.identifier?.id,version:"0.0.0",engines:{vscode:"*"}},'
    'location:{scheme:"file",authority:"",path:_r.dest,query:"",fragment:""},'
    'isApplicationScoped:false,isMachineScoped:!!t?.isMachineScoped,'
    'isPreReleaseVersion:false,hadPreReleaseVersion:false,updated:false,pinned:false};'
    'this._onDidInstallExtensions.fire([{identifier:e.identifier,source:e,local:_local,'
    'operation:2,profileLocation:t?.profileLocation}]);'
    'console.log("[Hyperlight] Install complete:",e.identifier?.id);'
    'return _local'
    '}catch(_err){console.error("[Hyperlight] Install error:",_err);throw _err}'
    '}'
    'return Promise.resolve(this.channel.call("installFromGallery",[e,t]))'
    '.then(i=>Xj(i,null))}'
)
if ifg_proxy_old in d:
    d = d.replace(ifg_proxy_old, ifg_proxy_new, 1)
    print("Patched IPC proxy installFromGallery (browser-side download)")
    count += 1
else:
    print("WARNING: IPC proxy installFromGallery pattern not found", file=sys.stderr)

# 5b. Bypass the aggregator's 3-arg installFromGallery.
#     It calls getManifest on the gallery service which routes through the
#     WebSocket proxy and hangs on single-vCPU.  Skip all checks and go
#     directly to the server's installFromGallery (our patched IPC proxy).
agg3_old = 'async installFromGallery(e,t,i){let n=await this.extensionGalleryService.getManifest(e,ye.None)'
agg3_new = (
    'async installFromGallery(e,t,i){'
    'let _s=this.extensionManagementServerService.remoteExtensionManagementServer'
    '||this.extensionManagementServerService.localExtensionManagementServer;'
    'if(_s&&e?.assets?.download?.uri){'
    'console.log("[Hyperlight] aggregator: bypassing getManifest, direct to proxy");'
    'return _s.extensionManagementService.installFromGallery(e,t||{})}'
    'let n=await this.extensionGalleryService.getManifest(e,ye.None)'
)
if agg3_old in d:
    d = d.replace(agg3_old, agg3_new, 1)
    print("Patched aggregator 3-arg installFromGallery bypass")
    count += 1
else:
    print("WARNING: aggregator 3-arg installFromGallery bypass not found", file=sys.stderr)

# 6. Patch readExtensionResource to use HTTP for all extensions.
#    On single-vCPU, the IPC file read (via _fileService.readFile) hangs
#    because the server is CPU-starved. The server's /vscode-remote-resource
#    endpoint CAN serve these files via HTTP without IPC contention.
#    Covers both user-installed (/data/extensions/) and built-in
#    (/opt/vscode-server/extensions/) extension resources.
rer_old = (
    'var Jft=class extends rLt{'
    'constructor(o,e,t,i,n,r,s){super(o,e,t,i,n,r,s)}'
    'async readExtensionResource(o){'
    'if(o=Cn.uriToBrowserUri(o),o.scheme!==X.http&&o.scheme!==X.https&&o.scheme!==X.data)'
    'return(await this._fileService.readFile(o)).value.toString()'
)
rer_new = (
    'var Jft=class extends rLt{'
    'constructor(o,e,t,i,n,r,s){super(o,e,t,i,n,r,s)}'
    'async readExtensionResource(o){'
    'let _p=typeof o?.path==="string"?o.path:"";'
    'if(_p.startsWith("/data/extensions/")||_p.startsWith("/opt/vscode-server/extensions/")){try{'
    'let _r=await fetch("/vscode-remote-resource?path="+encodeURIComponent(_p));'
    'if(_r.ok)return await _r.text()'
    '}catch(_e){}}'
    'if(o=Cn.uriToBrowserUri(o),o.scheme!==X.http&&o.scheme!==X.https&&o.scheme!==X.data)'
    'return(await this._fileService.readFile(o)).value.toString()'
)
if rer_old in d:
    d = d.replace(rer_old, rer_new, 1)
    print("Patched readExtensionResource HTTP bypass for installed extensions")
    count += 1
else:
    print("WARNING: readExtensionResource pattern not found", file=sys.stderr)


assert count >= 1, "No patches applied to workbench.js"
open(f, "w").write(d)
print(f"Total: {count} patches applied")
