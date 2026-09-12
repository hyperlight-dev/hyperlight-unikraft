"""Verify all pre-installed agent packages import correctly.

This catches missing shared libraries, broken installs, or packages
that silently fail to load. Every package in the agent Dockerfile's
pip install line must appear here.
"""
import sys

PACKAGES = [
    # Data science
    ("numpy", "numpy"),
    ("pandas", "pandas"),
    ("scipy", "scipy"),
    ("scikit-learn", "sklearn"),
    ("matplotlib", "matplotlib"),
    ("seaborn", "seaborn"),
    ("sympy", "sympy"),
    # Document processing
    ("pillow", "PIL"),
    ("openpyxl", "openpyxl"),
    ("python-docx", "docx"),
    ("python-pptx", "pptx"),
    ("pypdf", "pypdf"),
    ("pdfplumber", "pdfplumber"),
    ("reportlab", "reportlab"),
    # Web / HTTP
    ("requests", "requests"),
    ("httpx", "httpx"),
    ("beautifulsoup4", "bs4"),
    # Utilities
    ("python-dateutil", "dateutil"),
    ("pytz", "pytz"),
    ("tqdm", "tqdm"),
    ("jinja2", "jinja2"),
    ("lxml", "lxml"),
    ("pydantic", "pydantic"),
    ("tabulate", "tabulate"),
    ("chardet", "chardet"),
    ("pyyaml", "yaml"),
    # Stdlib modules that need shared libs
    ("ssl (stdlib)", "ssl"),
    ("sqlite3 (stdlib)", "sqlite3"),
    ("ctypes (stdlib)", "ctypes"),
    ("bz2 (stdlib)", "bz2"),
    ("lzma (stdlib)", "lzma"),
]

failed = []
for pip_name, import_name in PACKAGES:
    try:
        mod = __import__(import_name)
        version = getattr(mod, "__version__", "ok")
        print(f"  {pip_name}: {version}")
    except Exception as e:
        failed.append((pip_name, str(e)))
        print(f"  {pip_name}: FAILED ({e})")

print()
if failed:
    print(f"FAILED: {len(failed)}/{len(PACKAGES)} packages broken")
    for name, err in failed:
        print(f"  {name}: {err}")
    sys.exit(1)
else:
    print(f"ALL {len(PACKAGES)} packages verified")
