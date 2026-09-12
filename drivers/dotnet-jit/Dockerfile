# .NET JIT runtime for Hyperlight/Unikraft.
#
# The guest compiles and executes C# source code in-process using the
# Roslyn compilation API — same dispatch model as Python/Node where raw
# source goes in and output comes out.  No host-side compilation needed.
#
# Self-contained publish on Alpine (musl) keeps the rootfs
# self-contained with its own dynamic linker.  ReadyToRun (R2R)
# pre-compiles Roslyn methods to native code, avoiding massive JIT
# overhead in the unikernel.  Trimming is disabled because Roslyn
# uses reflection heavily.

FROM mcr.microsoft.com/dotnet/sdk:9.0-alpine AS build

# Build the .NET dispatch server (includes Roslyn for in-guest compilation)
WORKDIR /src
COPY drivers/dotnet-jit/HlDotnetDispatch/ ./
RUN dotnet publish -c Release -r linux-musl-x64 --self-contained -o /dispatch

# Compile the C driver
RUN apk add --no-cache gcc musl-dev

# Preserve directory layout so #include "../hl_fc.h" resolves
COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/dotnet-jit/hl_dotnetdriver.c /src/drivers/dotnet-jit/hl_dotnetdriver.c

RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_dotnetdriver /src/drivers/dotnet-jit/hl_dotnetdriver.c

# Minimal rootfs: .NET dispatch server + driver + musl
FROM scratch

# musl dynamic linker
COPY --from=build /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1

# C++ runtime libraries (needed by CoreCLR native code)
COPY --from=build /usr/lib/libstdc++.so.6 /usr/lib/libstdc++.so.6
COPY --from=build /usr/lib/libgcc_s.so.1 /usr/lib/libgcc_s.so.1

# OpenSSL libraries — .NET's crypto shim (libSystem.Security.Cryptography.
# Native.OpenSsl.so) requires libssl + libcrypto.  Without these,
# Guid.NewGuid() fails because it routes through OpenSSL's RAND_bytes().
COPY --from=build /usr/lib/libssl.so.3 /usr/lib/libssl.so.3
COPY --from=build /usr/lib/libcrypto.so.3 /usr/lib/libcrypto.so.3

# .NET dispatch server (CoreCLR + Roslyn + app, R2R pre-compiled)
COPY --from=build /dispatch/ /app/

# Hyperlight driver
COPY --from=build /hl_dotnetdriver /usr/local/bin/hl_dotnetdriver

# /tmp needed for runtime temp files
COPY --from=build /tmp /tmp
