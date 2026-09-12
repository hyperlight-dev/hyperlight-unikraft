# Kestrel (ASP.NET Core) HTTP server rootfs for Hyperlight.
#
# Publishes a self-contained, trimmed linux-musl-x64 app and bakes it onto the
# dotnet-aot driver rootfs (shared exec driver + musl linker + glibc compat
# symlinks). The guest runs the published native host binary directly.
#
# Build:  just build-rootfs http-kestrel examples/http-server/kestrel/Dockerfile
# Run:    hluk run --initrd build-elfloader/http-kestrel-rootfs.cpio \
#             --scratch-mb 128 --net --port 8080 --exec /app/KestrelHyperlight
FROM mcr.microsoft.com/dotnet/sdk:9.0-alpine AS build
WORKDIR /src
COPY examples/http-server/kestrel/Program.cs examples/http-server/kestrel/KestrelHyperlight.csproj ./
RUN dotnet publish -c Release -r linux-musl-x64 --self-contained \
    -p:PublishTrimmed=true \
    -p:InvariantGlobalization=true \
    -o /out

FROM hluk-dotnet-aot-rootfs:latest
COPY --from=build /out/ /app/
# C++ runtime the CoreCLR host needs (see the header note).
COPY --from=build /usr/lib/libstdc++.so.6 /usr/lib/libstdc++.so.6
COPY --from=build /usr/lib/libgcc_s.so.1 /usr/lib/libgcc_s.so.1
