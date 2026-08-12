FROM alpine:3.21 AS base
RUN apk add --no-cache libstdc++ libgcc libcrypto3 libssl3

FROM base AS download
RUN apk add --no-cache curl jq
RUN curl -fL --retry 3 --retry-delay 5 -o /tmp/powershell.tar.gz \
    https://github.com/PowerShell/PowerShell/releases/download/v7.5.4/powershell-7.5.4-linux-musl-x64.tar.gz
RUN mkdir -p /opt/microsoft/powershell/7 && \
    tar -xzf /tmp/powershell.tar.gz -C /opt/microsoft/powershell/7 && \
    cd /opt/microsoft/powershell/7 && \
    rm -rf ref/ Schemas/ && \
    rm -rf ru/ ja/ pl/ ko/ fr/ de/ it/ es/ pt-BR/ tr/ cs/ zh-Hans/ zh-Hant/ && \
    rm -f *.xml && \
    rm -rf Modules/PSReadLine Modules/PackageManagement Modules/PSResourceGet
# Enable invariant globalization (no ICU dependency)
RUN jq '.runtimeOptions.configProperties["System.Globalization.Invariant"] = true' \
    /opt/microsoft/powershell/7/pwsh.runtimeconfig.json > /tmp/runtimeconfig.json && \
    mv /tmp/runtimeconfig.json /opt/microsoft/powershell/7/pwsh.runtimeconfig.json
# Stub /proc/self/cmdline - PowerShell reads this to detect login shell
RUN mkdir -p /tmp/proc-stub/self && printf 'pwsh\0' > /tmp/proc-stub/self/cmdline
# Stub /etc/passwd and home directory - PowerShell needs user info for config paths
RUN mkdir -p /tmp/etc-stub && echo 'root:x:0:0:root:/root:/bin/sh' > /tmp/etc-stub/passwd
RUN mkdir -p /tmp/home-stub && touch /tmp/home-stub/.keep
# Create /tmp directory stub - PowerShell needs /tmp for module analysis cache
RUN mkdir -p /tmp/tmp-stub

FROM scratch AS rootfs
COPY --from=base /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
COPY --from=base /usr/lib/libstdc++.so.6 /usr/lib/libstdc++.so.6
COPY --from=base /usr/lib/libgcc_s.so.1 /usr/lib/libgcc_s.so.1
COPY --from=base /usr/lib/libcrypto.so.3 /usr/lib/libcrypto.so.3
COPY --from=base /usr/lib/libssl.so.3 /usr/lib/libssl.so.3
COPY --from=base /usr/lib/ossl-modules /usr/lib/ossl-modules
COPY --from=base /etc/ssl /etc/ssl
COPY --from=download /opt/microsoft/powershell/7 /opt/microsoft/powershell/7
COPY --from=download /tmp/proc-stub/self /proc/self
COPY --from=download /tmp/etc-stub/passwd /etc/passwd
COPY --from=download /tmp/home-stub /root
COPY --from=download /tmp/tmp-stub /tmp

# --- CPIO rootfs builder (used by: docker build --target cpio) ---
