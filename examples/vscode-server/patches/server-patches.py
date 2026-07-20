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
    'let _fs=await import("node:fs");'
    'let _path=await import("node:path");'
    'let _b=Buffer.from(n[0],"base64");'
    'let _p=_path.join("/data","_ext"+Date.now()+".vsix");'
    '_fs.writeFileSync(_p,_b);'
    'return{path:_p}'
    '}'
    'case"installChunkStart":{'
    'let _fs=await import("node:fs");'
    'let _path=await import("node:path");'
    'let _p=_path.join("/data","_ext_chunked_"+Date.now()+".vsix");'
    'this.__chunkPath=_p;this.__chunkFd=_fs.openSync(_p,"w");'
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
    'let _path=await import("node:path");'
    '_fs.closeSync(this.__chunkFd);'
    'let _p=this.__chunkPath;'
    'delete this.__chunkPath;delete this.__chunkFd;'
    'let yauzl=(await import("/opt/vscode-server/node_modules/yauzl/index.js")).default;'
    'let extDir="/data/extensions";'
    'let tmpDir=_path.join(extDir,"_tmp_"+Date.now());'
    '_fs.mkdirSync(tmpDir,{recursive:true});'
    'let installed=await new Promise((resolve,reject)=>{'
    'yauzl.open(_p,{lazyEntries:true},(err,zf)=>{'
    'if(err)return reject(err);'
    'let manifest=null;let pending=0;let done=false;'
    'function checkDone(){if(done&&pending===0){'
    'if(!manifest)return reject(new Error("no package.json"));'
    'let id=manifest.publisher+"."+manifest.name+"-"+manifest.version;'
    'let dest=_path.join(extDir,id);'
    'try{_fs.rmSync(dest,{recursive:true,force:true})}catch{}'
    '_fs.renameSync(tmpDir,dest);'
    'resolve({id,dest,manifest})}}'
    'zf.readEntry();'
    'zf.on("entry",(entry)=>{'
    'let rel=entry.fileName.replace(/^extension\\//,"");'
    'if(!rel||entry.fileName.endsWith("/")){zf.readEntry();return}'
    'pending++;'
    'zf.openReadStream(entry,(err,stream)=>{'
    'if(err){pending--;checkDone();zf.readEntry();return}'
    'if(rel==="package.json"){'
    'let chunks=[];'
    'stream.on("data",c=>chunks.push(c));'
    'stream.on("end",()=>{'
    'manifest=JSON.parse(Buffer.concat(chunks).toString());'
    'let out=_path.join(tmpDir,rel);'
    '_fs.mkdirSync(_path.dirname(out),{recursive:true});'
    '_fs.writeFileSync(out,Buffer.concat(chunks));'
    'pending--;checkDone();zf.readEntry()});'
    '}else{'
    'let out=_path.join(tmpDir,rel);'
    '_fs.mkdirSync(_path.dirname(out),{recursive:true});'
    'let ws=_fs.createWriteStream(out);'
    'stream.pipe(ws);'
    'ws.on("close",()=>{pending--;checkDone();zf.readEntry()});'
    '}'
    '});'
    '});'
    'zf.on("end",()=>{done=true;checkDone()});'
    '});'
    '});'
    'let ejPath=_path.join(extDir,"extensions.json");'
    'let exts=[];'
    'try{exts=JSON.parse(_fs.readFileSync(ejPath,"utf8"))}catch{}'
    'exts=exts.filter(e=>e.identifier?.id!==installed.manifest.publisher+"."+installed.manifest.name);'
    'exts.push({identifier:{id:installed.manifest.publisher+"."+installed.manifest.name},'
    'version:installed.manifest.version,'
    'location:{scheme:"file",path:installed.dest},'
    'relativeLocation:installed.id,'
    'metadata:{installedTimestamp:Date.now(),source:"gallery"}});'
    '_fs.writeFileSync(ejPath,JSON.stringify(exts));'
    'try{let cacheDir="/data/data/CachedProfilesData/__default__profile__";'
    'for(let cf of["extensions.user.cache","extensions.builtin.cache"])'
    '{let cp=_path.join(cacheDir,cf);try{_fs.unlinkSync(cp)}catch{}}'
    '}catch{}'
    'return{path:installed.dest,id:installed.id,vsix:_p}'
    '}'
    'case"installFromGallery":return this.service.installFromGallery(n[0],ef(n[1],o));case"installGalleryExtensions"'
)
if ifb_old in d:
    d = d.replace(ifb_old, ifb_new, 1)
    print("Patched installFromBuffer IPC handler")
    count += 1
else:
    print("WARNING: installFromBuffer anchor pattern not found", file=sys.stderr)

# 3. Bypass extension signature verification (vsce-sign is a native module
#    that can't load in the musl unikernel). When the module fails to import,
#    verify() returns undefined → caller throws SignatureVerificationInternal.
#    Patch to return "Success" instead so the install proceeds.
sig_old = (
    'this.logService.info(`Extension signature verification is not done: ${t}`);return}'
)
sig_new = (
    'this.logService.info(`Extension signature verification is not done: ${t}`);return"Success"}'
)
if sig_old in d:
    d = d.replace(sig_old, sig_new, 1)
    print("Patched signature verification bypass (import path)")
    count += 1
else:
    print("WARNING: signature verification bypass (import) pattern not found", file=sys.stderr)

# Also patch the verify() call failure path: when the native verify()
# throws, VS Code sets code="UnknownError". Replace with "Success".
sig2_old = 'l={code:"UnknownError",didExecute:!1,output:'
sig2_new = 'l={code:"Success",didExecute:!1,output:'
if sig2_old in d:
    d = d.replace(sig2_old, sig2_new, 1)
    print("Patched signature verification bypass (verify call path)")
    count += 1
else:
    print("WARNING: signature verification bypass (verify call) pattern not found", file=sys.stderr)

assert count >= 1, "No patches applied to server-main.js"
open(f, "w").write(d)
print(f"Total: {count} server-main.js patches applied")
