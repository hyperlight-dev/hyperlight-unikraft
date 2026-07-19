"""Patch server-main.js for single-vCPU cooperative scheduling."""
import sys

f = "/rootfs/opt/vscode-server/out/server-main.js"
d = open(f).read()
count = 0

# 1. Bypass getExtensionsControlManifest (fetches from CDN, blocks single-vCPU).
old_ecm = 'async getExtensionsControlManifest(){if(!await'
new_ecm = 'async getExtensionsControlManifest(){return{malicious:[],deprecated:{},search:[],autoUpdate:{}}}async _ecm_disabled(){if(!await'
if old_ecm in d:
    d = d.replace(old_ecm, new_ecm, 1)
    print("Patched server getExtensionsControlManifest bypass")
    count += 1
else:
    print("WARNING: server getExtensionsControlManifest pattern not found", file=sys.stderr)

old_j7 = 'getExtensionsControlManifest(){let e=new Date'
new_j7 = 'getExtensionsControlManifest(){return Promise.resolve({malicious:[],deprecated:{},search:[],autoUpdate:{}})}__ecm_cached(){let e=new Date'
if old_j7 in d:
    d = d.replace(old_j7, new_j7, 1)
    print("Patched server J7 cached getExtensionsControlManifest bypass")
    count += 1
else:
    print("WARNING: server J7 cached getExtensionsControlManifest pattern not found", file=sys.stderr)

# 2. Add chunked install IPC handlers: the browser downloads the VSIX,
#    sends it in 512KB base64 chunks, then triggers the install.
#    This avoids the WebSocket message-size limit (~1MB) that blocks
#    large extensions from being sent in a single IPC call.
ifb_old = 'case"installFromGallery":return this.service.installFromGallery(n[0],ef(n[1],o));case"installGalleryExtensions"'
ifb_new = (
    'case"installFromBuffer":{'
    'console.log("[installFromBuffer] entered, payload length:",n[0]?.length);'
    'let _fs=await import("node:fs");'
    'let _os=await import("node:os");'
    'let _path=await import("node:path");'
    'let _b=Buffer.from(n[0],"base64");'
    'let _p=_path.join(_os.tmpdir(),"_ext"+Date.now()+".vsix");'
    '_fs.writeFileSync(_p,_b);'
    'console.log("[installFromBuffer] wrote",_b.length,"bytes to",_p);'
    'return{path:_p}'
    '}'
    'case"installChunkStart":{'
    'let _fs=await import("node:fs");'
    'let _os=await import("node:os");'
    'let _path=await import("node:path");'
    'let _p=_path.join(_os.tmpdir(),"_ext_chunked_"+Date.now()+".vsix");'
    'this.__chunkPath=_p;this.__chunkFd=_fs.openSync(_p,"w");'
    'console.log("[installChunk] start:",_p);'
    'return{path:_p}'
    '}'
    'case"installChunkData":{'
    'let _fs=await import("node:fs");'
    'let _b=Buffer.from(n[0],"base64");'
    '_fs.writeSync(this.__chunkFd,_b);'
    'return{ok:true,written:_b.length}'
    '}'
    'case"installChunkEnd":{'
    'let _fs=await import("node:fs");'
    '_fs.closeSync(this.__chunkFd);'
    'console.log("[installChunk] end, file ready at:",this.__chunkPath);'
    'let _p=this.__chunkPath;'
    'delete this.__chunkPath;delete this.__chunkFd;'
    'return{path:_p}'
    '}'
    'case"installFromGallery":return this.service.installFromGallery(n[0],ef(n[1],o));case"installGalleryExtensions"'
)
if ifb_old in d:
    d = d.replace(ifb_old, ifb_new, 1)
    print("Patched installFromBuffer IPC handler")
    count += 1
else:
    print("WARNING: installFromBuffer anchor pattern not found", file=sys.stderr)

assert count >= 1, "No patches applied to server-main.js"
open(f, "w").write(d)
print(f"Total: {count} server-main.js patches applied")
