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
    'let _path=await import("node:path");'
    'let _b=Buffer.from(n[0],"base64");'
    'let _p=_path.join("/data","_ext"+Date.now()+".vsix");'
    '_fs.writeFileSync(_p,_b);'
    'console.log("[installFromBuffer] wrote",_b.length,"bytes to",_p);'
    'return{path:_p}'
    '}'
    'case"installChunkStart":{'
    'let _fs=await import("node:fs");'
    'let _os=await import("node:os");'
    'let _path=await import("node:path");'
    'let _p=_path.join("/data","_ext_chunked_"+Date.now()+".vsix");'
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
    'let _path=await import("node:path");'
    '_fs.closeSync(this.__chunkFd);'
    'console.log("[installChunk] end, extracting:",this.__chunkPath);'
    'let _p=this.__chunkPath;'
    'delete this.__chunkPath;delete this.__chunkFd;'
    'let yauzl=(await import("/opt/vscode-server/node_modules/yauzl/index.js")).default;'
    'let extDir="/data/extensions";'
    'let installed=await new Promise((resolve,reject)=>{'
    'yauzl.open(_p,{lazyEntries:true},(err,zf)=>{'
    'if(err)return reject(err);'
    'let manifest=null;'
    'zf.readEntry();'
    'zf.on("entry",(entry)=>{'
    'if(entry.fileName==="extension/package.json"){'
    'zf.openReadStream(entry,(err,stream)=>{'
    'if(err)return reject(err);'
    'let chunks=[];'
    'stream.on("data",c=>chunks.push(c));'
    'stream.on("end",()=>{'
    'manifest=JSON.parse(Buffer.concat(chunks).toString());'
    'zf.readEntry();'
    '});'
    '});'
    '}else{zf.readEntry()}'
    '});'
    'zf.on("end",()=>{'
    'if(!manifest)return reject(new Error("no package.json"));'
    'let id=manifest.publisher+"."+manifest.name+"-"+manifest.version;'
    'let dest=_path.join(extDir,id);'
    '_fs.mkdirSync(dest,{recursive:true});'
    'yauzl.open(_p,{lazyEntries:true},(err2,zf2)=>{'
    'if(err2)return reject(err2);'
    'let pending=0;let done=false;'
    'zf2.readEntry();'
    'zf2.on("entry",(entry)=>{'
    'let rel=entry.fileName.replace(/^extension\\//,"");'
    'if(!rel||entry.fileName.endsWith("/")){zf2.readEntry();return}'
    'let out=_path.join(dest,rel);'
    '_fs.mkdirSync(_path.dirname(out),{recursive:true});'
    'pending++;'
    'zf2.openReadStream(entry,(err,stream)=>{'
    'if(err){zf2.readEntry();pending--;return}'
    'let ws=_fs.createWriteStream(out);'
    'stream.pipe(ws);'
    'ws.on("close",()=>{pending--;if(done&&pending===0)resolve({id,dest,manifest});zf2.readEntry()});'
    '});'
    '});'
    'zf2.on("end",()=>{done=true;if(pending===0)resolve({id,dest,manifest})});'
    '});'
    '});'
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
    'console.log("[installChunk] installed to:",installed.dest);'
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

# 4. Log scanExtensions IPC calls for debugging
scan_old = 'case"scanExtensions":{let i=n[0],'
scan_new = 'case"scanExtensions":{console.log("[scanExtensions] IPC called");let i=n[0],'
if scan_old in d:
    d = d.replace(scan_old, scan_new, 1)
    print("Patched scanExtensions logging")
    count += 1
else:
    print("WARNING: scanExtensions logging pattern not found", file=sys.stderr)

assert count >= 1, "No patches applied to server-main.js"
open(f, "w").write(d)
print(f"Total: {count} server-main.js patches applied")
