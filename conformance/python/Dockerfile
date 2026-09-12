# Conformance rootfs: Python + CPython test suite.
#
# Reuses the base Python driver image and layers the CPython Lib/test/
# suite on top so we can run `import test.test_<module>` inside the guest.
#
# Build:  just build-conformance python
# Run:    just conformance python

FROM hluk-python-rootfs AS base

FROM python:3.12-slim-bookworm AS testsuite

RUN apt-get update && apt-get install -y --no-install-recommends \
        wget xz-utils && \
    rm -rf /var/lib/apt/lists/*

# Grab the CPython test suite matching our interpreter version.
RUN PY_VER=$(python3 -c 'import sys; print(f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}")') && \
    wget -q "https://www.python.org/ftp/python/${PY_VER}/Python-${PY_VER}.tar.xz" -O /tmp/cpython.tar.xz && \
    mkdir /tmp/cpython && \
    tar -xf /tmp/cpython.tar.xz -C /tmp/cpython --strip-components=1 && \
    rm -rf /tmp/cpython.tar.xz

# Final rootfs: base driver image + test suite layered on top
FROM base
COPY --from=testsuite /tmp/cpython/Lib/test /usr/local/lib/python3.12/test
