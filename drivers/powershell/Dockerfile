# PowerShell runtime for Hyperlight/Unikraft.
#
# Installs PowerShell 7 (which ships .NET CoreCLR) and bundles it
# with a dedicated driver that writes PS1 code to a temp file and
# runs it via vfork+execve of pwsh.
#
# The rootfs is large (~180 MB) due to the full .NET runtime.

FROM mcr.microsoft.com/powershell:7.4-alpine-3.20 AS pwsh-base

# Create dirs that FROM scratch doesn't have
RUN mkdir -p /empty-tmp /stub-proc/self && \
    printf 'pwsh\0-NoProfile\0-File\0/tmp/hl_dispatch.ps1\0' > /stub-proc/self/cmdline

# ── Compile the driver ──────────────────────────────────────────
FROM alpine:3.20 AS build-driver

RUN apk add --no-cache gcc musl-dev

COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/powershell/hl_pwshdriver.c /src/drivers/powershell/hl_pwshdriver.c
RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_pwshdriver /src/drivers/powershell/hl_pwshdriver.c

# ── Rootfs ──────────────────────────────────────────────────────
FROM scratch

# musl dynamic linker (pwsh's native libs need it)
COPY --from=pwsh-base /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1

# Core system libraries pwsh depends on
COPY --from=pwsh-base /usr/lib/libstdc++.so* /usr/lib/
COPY --from=pwsh-base /usr/lib/libgcc_s.so* /usr/lib/
COPY --from=pwsh-base /lib/libz.so* /lib/

# PowerShell + .NET runtime
COPY --from=pwsh-base /opt/microsoft/powershell/7/ /opt/microsoft/powershell/7/

# Minimal /etc + /tmp
COPY --from=pwsh-base /etc/passwd /etc/passwd
COPY --from=pwsh-base /etc/group /etc/group
COPY --from=pwsh-base /empty-tmp/ /tmp/
COPY --from=pwsh-base /stub-proc/ /proc/

# Driver
COPY --from=build-driver /hl_pwshdriver /usr/local/bin/hl_pwshdriver
