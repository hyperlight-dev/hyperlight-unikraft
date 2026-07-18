"""Patch server-main.js for single-vCPU cooperative scheduling."""
import sys

f = "/rootfs/opt/vscode-server/out/server-main.js"
d = open(f).read()
count = 0

# 1. Bypass getExtensionsControlManifest (fetches from CDN, blocks single-vCPU).
#    Same approach as the workbench.js patch: return empty manifest immediately.
#    Patch the gallery service async implementation:
old_ecm = 'async getExtensionsControlManifest(){if(!await'
new_ecm = 'async getExtensionsControlManifest(){return{malicious:[],deprecated:{},search:[],autoUpdate:{}}}async _ecm_disabled(){if(!await'
if old_ecm in d:
    d = d.replace(old_ecm, new_ecm, 1)
    print("Patched server getExtensionsControlManifest bypass")
    count += 1
else:
    print("WARNING: server getExtensionsControlManifest pattern not found", file=sys.stderr)

#    Also patch the J7 cached version to always return the empty manifest:
old_j7 = 'getExtensionsControlManifest(){let e=new Date'
new_j7 = 'getExtensionsControlManifest(){return Promise.resolve({malicious:[],deprecated:{},search:[],autoUpdate:{}})}__ecm_cached(){let e=new Date'
if old_j7 in d:
    d = d.replace(old_j7, new_j7, 1)
    print("Patched server J7 cached getExtensionsControlManifest bypass")
    count += 1
else:
    print("WARNING: server J7 cached getExtensionsControlManifest pattern not found", file=sys.stderr)

assert count >= 1, "No patches applied to server-main.js"
open(f, "w").write(d)
print(f"Total: {count} server-main.js patches applied")
