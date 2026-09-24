# Module loading is limited in the unikernel rootfs, so write with
# [Console] rather than Write-Host.
[Console]::WriteLine("Hello from {{name}}!")
[Console]::WriteLine("PowerShell $($PSVersionTable.PSVersion), inside a Hyperlight micro-VM")
