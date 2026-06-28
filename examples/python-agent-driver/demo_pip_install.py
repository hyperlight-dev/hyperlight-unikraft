import sys, os
os.environ["PIP_NO_INPUT"] = "1"
os.environ["PIP_DISABLE_PIP_VERSION_CHECK"] = "1"
os.environ["PIP_NO_CACHE_DIR"] = "1"

from pip._internal.cli.main import main as pip_main
rc = pip_main(["install", "--target", "/tmp/pip_packages", "six"])
if rc != 0:
    sys.exit(rc)

sys.path.insert(0, "/tmp/pip_packages")
import six
print(f"Installed and imported six {six.__version__}")
