# python-agent-driver — Python runtime driver for Hyperlight.
#
# Same deps as python-agent (numpy + pandas + the usual stack) PLUS
# a small C program `hl_pydriver` that embeds libpython3.12, does
# Py_Initialize + warm-up imports on its first invocation, and then
# installs an FC-aware dispatch callback that handles every
# subsequent host->guest call without re-running main().

ARG BASE=ghcr.io/hyperlight-dev/hyperlight-unikraft/python-base:latest

# Stage 1: Python deps (same as python-agent).
FROM python:3.12-slim AS deps
RUN pip install --target=/deps --no-cache-dir --prefer-binary \
    tqdm pyyaml jinja2 beautifulsoup4 tabulate click tenacity \
    python-dotenv pypdf openpyxl markdown-it-py pydantic pillow \
    lxml cryptography python-dateutil numpy pandas \
    python-docx python-pptx \
    chardet charset-normalizer \
    requests httpx aiohttp \
    feedparser markdown markdownify \
    Faker pycountry \
    loguru schedule send2trash \
    duckdb polars \
    xlrd xlsxwriter pyxlsb odfpy \
    pdfplumber pdfrw PyPDF2 \
    qrcode svgwrite \
    rapidfuzz \
    networkx sympy \
    pydub srt mutagen \
    plotly altair bokeh \
    statsmodels scikit-learn scipy \
    wordcloud \
    nltk textblob gensim \
    fastapi uvicorn hypercorn \
    typer pygments platformdirs distro \
    pytest pytest-cov pytest-asyncio coverage hypothesis \
    ruff pylint pyflakes bandit vulture radon rope \
    websockets \
    fpdf2 reportlab \
    APScheduler celery \
    numpy-financial docx2txt pipdeptree watchdog rarfile \
    boto3 google-api-python-client slack-sdk praw tweepy \
    scrapy trafilatura builtwith exchange-calendars \
    paramiko fabric pexpect gitpython
RUN set -eux; \
    find /deps -maxdepth 1 -type d -name '*.dist-info' -exec rm -rf {} + 2>/dev/null || true; \
    find /deps \( -type d -name tests -o -type d -name test \) -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type f -name '*.pyc' -delete 2>/dev/null || true; \
    find /deps -name '*.so*' -exec strip --strip-unneeded {} + 2>/dev/null || true

# Stage 2: build hl_pydriver against the SAME Python 3.12-from-source
# that produces local-python-base, so the glibc/libpython3.12.so ABI
# matches the runtime.
FROM local-python-base-dev AS driver-build
COPY hl_pydriver.c /src/hl_pydriver.c
RUN PY_INC=$(python3.12 -c 'import sysconfig; print(sysconfig.get_path("include"))') && \
    PY_LIBDIR=$(python3.12 -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))') && \
    gcc -O2 -fPIE -pie -Wl,-E \
        -fno-stack-protector \
        -I$PY_INC \
        -o /src/hl_pydriver /src/hl_pydriver.c \
        -L$PY_LIBDIR \
        -Wl,-rpath,/usr/local/lib \
        -Wl,-rpath,/lib/x86_64-linux-gnu \
        -lpython3.12 -ldl -lpthread -lm -lutil

# Stage 3: assemble rootfs.
FROM ${BASE} AS rootfs
COPY --from=deps         /deps            /usr/local/lib/python3.12/site-packages
COPY --from=driver-build /src/hl_pydriver /bin/hl_pydriver
COPY pydoc_stub.py /usr/local/lib/python3.12/pydoc.py
# pip for in-guest package installation (vfork+execve subprocess)
COPY --from=deps /usr/local/lib/python3.12/site-packages/pip /usr/local/lib/python3.12/site-packages/pip
COPY --from=deps /usr/local/lib/python3.12/ensurepip /usr/local/lib/python3.12/ensurepip
# stdlib modules needed by pip but missing from python-base
COPY --from=deps /usr/local/lib/python3.12/xmlrpc /usr/local/lib/python3.12/xmlrpc
COPY --from=deps /usr/local/lib/python3.12/zipapp.py /usr/local/lib/python3.12/zipapp.py
# DNS-over-TCP resolver and pip target path (vfork subprocess support)
COPY rootfs/usr/local/lib/python3.12/sitecustomize.py /usr/local/lib/python3.12/sitecustomize.py
COPY rootfs/etc/pip.conf /etc/pip.conf

# Stage 3b: build static PIE utility binaries (elfloader only supports ET_DYN).
FROM debian:bookworm-slim AS utils-build
RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev make wget bzip2 ca-certificates && \
    rm -rf /var/lib/apt/lists/*
RUN printf '#include <unistd.h>\n#include <string.h>\nint main(int c,char**v){int i;for(i=1;i<c;i++){if(i>1)write(1," ",1);write(1,v[i],strlen(v[i]));}write(1,"\\n",1);return 0;}\n' > /echo.c && \
    gcc -O2 -static-pie -o /echo /echo.c
RUN printf 'int main(){return 0;}\n' > /true.c && \
    gcc -O2 -static-pie -o /true /true.c
# Build busybox as static-pie (elfloader requires ET_DYN, so plain -static won't work).
RUN wget -q https://busybox.net/downloads/busybox-1.36.1.tar.bz2 && \
    tar xf busybox-1.36.1.tar.bz2 && cd busybox-1.36.1 && \
    make defconfig && \
    sed -i 's/CONFIG_TC=y/# CONFIG_TC is not set/' .config && \
    sed -i 's/# CONFIG_STATIC is not set/CONFIG_STATIC=y/' .config && \
    sed -i 's|CONFIG_EXTRA_CFLAGS=""|CONFIG_EXTRA_CFLAGS="-fPIE -fPIC"|' .config && \
    make -j$(nproc) && \
    gcc -fPIE -fPIC -static-pie -o /busybox \
      -Wl,--sort-common -Wl,--sort-section -Wl,alignment \
      -Wl,--start-group \
      applets/built-in.o \
      $(find . -name 'lib.a' -path './*/lib.a' | sort) \
      -Wl,--end-group \
      -lm -lresolv && \
    strip /busybox

# Stage 4: pack CPIO.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils ca-certificates
COPY --from=rootfs / /rootfs/
COPY --from=utils-build /echo /rootfs/bin/echo
COPY --from=utils-build /true /rootfs/bin/true
COPY --from=utils-build /busybox /rootfs/bin/busybox
RUN cd /rootfs && for cmd in sh cat ls cp mv rm mkdir rmdir ln head tail wc \
        grep sed awk sort uniq tr cut tee xargs find test basename dirname \
        env which id whoami uname hostname date sleep; do \
        ln -sf busybox bin/$cmd; \
    done

# DNS configuration for glibc's getaddrinfo
RUN mkdir -p /rootfs/etc && \
    echo "nameserver 8.8.8.8" > /rootfs/etc/resolv.conf && \
    echo "nameserver 8.8.4.4" >> /rootfs/etc/resolv.conf && \
    echo "hosts: files dns" > /rootfs/etc/nsswitch.conf && \
    echo "127.0.0.1 localhost" > /rootfs/etc/hosts

# CA certificates for TLS
RUN mkdir -p /rootfs/etc/ssl/certs /rootfs/usr/lib/ssl && \
    cp /etc/ssl/certs/ca-certificates.crt /rootfs/etc/ssl/certs/ && \
    ln -sf /etc/ssl/certs/ca-certificates.crt /rootfs/usr/lib/ssl/cert.pem

RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
