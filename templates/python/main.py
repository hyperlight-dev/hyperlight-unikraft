import platform
import sys

print(f"Hello from {{name}}!")
print(f"Python {sys.version_info.major}.{sys.version_info.minor} on {platform.system()} "
      f"({platform.machine()}), inside a Hyperlight micro-VM")
