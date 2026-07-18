# vscode-server — VS Code Server (REH-Web) for Hyperlight.
#
# Uses Alpine's musl PIE Node.js binary (required by elfloader) with
# the VS Code Server JS files. Native .node modules won't load (glibc
# ABI mismatch) but the server has fallbacks for all of them.

# Stage 1: Download VS Code Server (REH-Web) for Linux x64.
FROM ubuntu:22.04 AS download
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates
RUN curl -L "https://update.code.visualstudio.com/latest/server-linux-x64-web/stable" \
        -o /tmp/vscode-server.tar.gz && \
    mkdir -p /opt/vscode-server && \
    tar -xzf /tmp/vscode-server.tar.gz -C /opt/vscode-server --strip-components=1 && \
    rm /tmp/vscode-server.tar.gz

# Stage 2: Get musl PIE Node.js from Alpine.
FROM node:22-alpine AS node
RUN apk add --no-cache binutils && strip /usr/local/bin/node

# Stage 3: Rootfs assembly.
FROM scratch AS rootfs

# Musl dynamic linker + shared libraries
COPY --from=node /lib/ld-musl-x86_64.so.1    /lib/ld-musl-x86_64.so.1
COPY --from=node /usr/lib/libstdc++.so.6      /usr/lib/libstdc++.so.6
COPY --from=node /usr/lib/libgcc_s.so.1       /usr/lib/libgcc_s.so.1

# VS Code Server (JS files, extensions, resources)
COPY --from=download /opt/vscode-server /opt/vscode-server

# Replace bundled glibc Node.js with musl PIE Node.js
COPY --from=node /usr/local/bin/node /opt/vscode-server/node

# Busybox: use the static-PIE build from the shell-base image.
# Alpine's busybox-static is ET_EXEC (non-PIE) which the elfloader
# rejects; the shell-base image has a static-PIE (ET_DYN) build.
COPY --from=ghcr.io/hyperlight-dev/hyperlight-unikraft/shell-base:latest \
     /bin/busybox /bin/busybox

# Stage 4: Pack CPIO.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils ca-certificates python3
COPY --from=rootfs / /rootfs/

# Patch VS Code Server JS for single-vCPU cooperative scheduling:
# 1. Gallery manifest barrier bypass: construct manifest inline instead of
#    blocking forever on browser IPC that can't arrive on single-vCPU.
#    The manifest is a static capabilities descriptor identical to what the
#    base class builds from product.json — no network calls required.
RUN sed -i 's|async getExtensionGalleryManifest(){return await this.barrier.wait(),this._extensionGalleryManifest??null}|async getExtensionGalleryManifest(){if(!this._extensionGalleryManifest){let _g=this.productService?.extensionsGallery;if(!_g){try{_g=(await super.getExtensionGalleryManifest())?.extensionsGallery}catch{}}let _s=_g?.serviceUrl??"https://marketplace.visualstudio.com/_apis/public/gallery";let _r=[{id:_s+"/extensionquery",type:"ExtensionQueryService"},{id:_s+"/vscode/{publisher}/{name}/latest",type:"ExtensionLatestVersionUriTemplate"},{id:_s+"/publishers/{publisher}/extensions/{name}/{version}/stats?statType={statTypeName}",type:"ExtensionStatisticsUriTemplate"}];this._extensionGalleryManifest={version:"",resources:_r,capabilities:{extensionQuery:{filtering:[{name:"Tag",value:1},{name:"ExtensionId",value:4},{name:"Category",value:5},{name:"ExtensionName",value:7},{name:"Target",value:8},{name:"Featured",value:9},{name:"SearchText",value:10},{name:"ExcludeWithFlags",value:12}],sorting:[{name:"NoneOrRelevance",value:0},{name:"LastUpdatedDate",value:1},{name:"Title",value:2},{name:"PublisherName",value:3},{name:"InstallCount",value:4},{name:"AverageRating",value:6},{name:"PublishedDate",value:10},{name:"WeightedRating",value:12}],flags:[{name:"None",value:0},{name:"IncludeVersions",value:1},{name:"IncludeFiles",value:2},{name:"IncludeCategoryAndTags",value:4},{name:"IncludeSharedAccounts",value:8},{name:"IncludeVersionProperties",value:16},{name:"ExcludeNonValidated",value:32},{name:"IncludeInstallationTargets",value:64},{name:"IncludeAssetUri",value:128},{name:"IncludeStatistics",value:256},{name:"IncludeLatestVersionOnly",value:512},{name:"Unpublished",value:4096},{name:"IncludeNameConflictInfo",value:32768},{name:"IncludeLatestPrereleaseAndStableVersionOnly",value:65536}]},signing:{allPublicRepositorySigned:true}}};this.barrier.open()}return this._extensionGalleryManifest}|g' \
        /rootfs/opt/vscode-server/out/server-main.js
# 2. Gallery config: always pass extensionsGallery to the browser.
#    The original code gates it on _webExtensionResourceUrlTemplate (a
#    resource-proxy URL), which may be unset in our environment.  The
#    browser just needs the serviceUrl to query the marketplace.
COPY patches/gallery-config.py /tmp/gallery-config.py
RUN python3 /tmp/gallery-config.py
# 3. WebSocket heartbeat timeout 20s -> 600s (server-side)
RUN sed -i \
        -e 's|e>=2e4\&\&n>=2e4\&\&o>=2e4\&\&!this._loadEstimator|e>=6e5\&\&n>=6e5\&\&o>=6e5\&\&!this._loadEstimator|g' \
        -e 's|e>=2e4\&\&n>=2e4\&\&!this._loadEstimator|e>=6e5\&\&n>=6e5\&\&!this._loadEstimator|g' \
        -e 's|Math.max(2e4-e,2e4-n,2e4-o,500)|Math.max(6e5-e,6e5-n,6e5-o,500)|g' \
        /rootfs/opt/vscode-server/out/server-main.js
# 4. Server-side patches: bypass IPC-blocking calls in install flow
COPY patches/server-patches.py /tmp/server-patches.py
RUN python3 /tmp/server-patches.py
# 5. Workbench patches: common headers, heartbeat, install flow
COPY patches/workbench-patches.py /tmp/workbench-patches.py
RUN python3 /tmp/workbench-patches.py

# DNS configuration
RUN mkdir -p /rootfs/etc && \
    echo "nameserver 8.8.8.8" > /rootfs/etc/resolv.conf && \
    echo "nameserver 8.8.4.4" >> /rootfs/etc/resolv.conf && \
    echo "hosts: files dns" > /rootfs/etc/nsswitch.conf && \
    echo "127.0.0.1 localhost" > /rootfs/etc/hosts

# CA certificates for TLS
RUN mkdir -p /rootfs/etc/ssl/certs /rootfs/usr/lib/ssl && \
    cp /etc/ssl/certs/ca-certificates.crt /rootfs/etc/ssl/certs/ && \
    ln -sf /etc/ssl/certs/ca-certificates.crt /rootfs/usr/lib/ssl/cert.pem

# Busybox applet symlinks
RUN for cmd in sh echo ls cat grep find wc head tail sort cut sed awk \
               cp mv rm mkdir rmdir ln basename dirname env sleep date \
               hostname id uname whoami which test true tr uniq tee xargs; do \
        ln -sf busybox /rootfs/bin/$cmd; \
    done

# VS Code Server expects bin/remote-cli/node for the extension host
# process.  Normally created at runtime, but cpiovfs is read-only.
RUN ln -sf ../../node /rootfs/opt/vscode-server/bin/remote-cli/node

# musl libc SONAME symlink: ld-musl-x86_64.so.1 IS the libc but Node.js
# DT_NEEDED lists libc.musl-x86_64.so.1; the second-instance dynamic
# linker must find it by that name.
RUN ln -sf ld-musl-x86_64.so.1 /rootfs/lib/libc.musl-x86_64.so.1

# Keepalive timer: prevents the Node.js event loop from blocking
# indefinitely on epoll_wait.  On the cooperative scheduler the server
# thread is removed from the run queue while blocked; a short timer
# gives it a wakeup_time so the scheduler re-queues it periodically.
RUN echo 'setInterval(()=>{},100);' > /rootfs/opt/vscode-server/keepalive.cjs

# Writable directories
RUN mkdir -p /rootfs/tmp /rootfs/data

RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
