# .NET Native AOT on Hyperlight

Publish a self-contained Native AOT binary targeting Alpine (musl):

```bash
dotnet publish -c Release -r linux-musl-x64 -o ./publish
```

The project file (`Hello.csproj`) must enable AOT compilation:

```xml
<PropertyGroup>
  <OutputType>Exe</OutputType>
  <TargetFramework>net9.0</TargetFramework>
  <ImplicitUsings>enable</ImplicitUsings>
  <PublishAot>true</PublishAot>
  <StripSymbols>true</StripSymbols>
  <InvariantGlobalization>true</InvariantGlobalization>
</PropertyGroup>
```

Build dependencies (Alpine): `clang gcc musl-dev zlib-dev`

**Note:** AOT binaries on Alpine link against musl.  The dotnet-aot rootfs includes the musl dynamic linker.

Run it by mounting the publish directory:

```bash
hluk run --initrd ../../build-elfloader/dotnet-aot-rootfs.cpio --scratch-mb 256 \
         --mount ./publish:/mnt/bin --exec /mnt/bin/Hello
```

Or with a snapshot (save once, run many):

```bash
# Save a snapshot with the mount point configured
hluk snapshot save --initrd ../../build-elfloader/dotnet-aot-rootfs.cpio --scratch-mb 256 \
                   --mount ./publish:/mnt/bin -o ../../.snapshots/dotnet-aot

# Run from snapshot — mount the publish directory
hluk snapshot run ../../.snapshots/dotnet-aot --mount ./publish:/mnt/bin --exec /mnt/bin/Hello
```
