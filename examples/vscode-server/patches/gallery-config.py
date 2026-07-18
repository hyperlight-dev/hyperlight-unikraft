"""Patch server-main.js to always pass extensionsGallery to the browser.

The original code gates extensionsGallery on _webExtensionResourceUrlTemplate
(a resource-proxy URL for web extensions), which may be unset in our
environment.  The browser only needs the serviceUrl to query the marketplace.
"""
import re

f = "/rootfs/opt/vscode-server/out/server-main.js"
d = open(f).read()

old = (
    'extensionsGallery:this._webExtensionResourceUrlTemplate'
    '&&this._productService.extensionsGallery'
    '?{...this._productService.extensionsGallery,'
    'resourceUrlTemplate:this._webExtensionResourceUrlTemplate'
    '.with({scheme:"http",authority:c,'
    'path:`${y}/${this._webExtensionResourceUrlTemplate.authority}'
    '${this._webExtensionResourceUrlTemplate.path}`})'
    '.toString(!0)}:void 0'
)

new = 'extensionsGallery:this._productService.extensionsGallery||void 0'

assert old in d, "extensionsGallery conditional pattern not found"
d = d.replace(old, new)
print("Patched productConfiguration.extensionsGallery")
open(f, "w").write(d)
