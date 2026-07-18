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

# 2. Add installFromBuffer IPC handler: accepts base64-encoded VSIX from
#    browser, writes to temp file, installs locally.  Avoids the server
#    having to download from the marketplace CDN while CPU-starved.
ifb_old = 'case"installFromGallery":return this.service.installFromGallery(n[0],ef(n[1],o));case"installGalleryExtensions"'
ifb_new = (
    'case"installFromBuffer":{'
    'console.log("[installFromBuffer] entered, payload length:",n[0]?.length);'
    'try{'
    'let _fs=await import("node:fs");'
    'let _os=await import("node:os");'
    'let _path=await import("node:path");'
    'let _b=Buffer.from(n[0],"base64");'
    'console.log("[installFromBuffer] decoded",_b.length,"bytes");'
    'let _p=_path.join(_os.tmpdir(),"_ext"+Date.now()+".vsix");'
    '_fs.writeFileSync(_p,_b);'
    'console.log("[installFromBuffer] wrote to",_p);'
    'let _r=await this.service.install(I.file(_p));'
    'console.log("[installFromBuffer] install OK");'
    'try{_fs.unlinkSync(_p)}catch(_e){}'
    'return _r'
    '}catch(_err){'
    'console.error("[installFromBuffer] ERROR:",_err?.message||_err);'
    'throw _err'
    '}'
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
