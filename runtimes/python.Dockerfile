# https://github.com/revsys/optimized-python-docker/blob/master/Dockerfile
# https://github.com/docker-library/python/blob/master/3.12/slim-bullseye/Dockerfile

FROM ubuntu:22.04 as python-dev

ENV PATH /usr/local/bin:$PATH
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

RUN set -xe ; \
	apt update ; \
	apt install -yqq --no-install-recommends build-essential make curl tar ca-certificates \
		libexpat-dev zlib1g-dev libffi-dev libssl-dev \
		libbz2-dev liblzma-dev libreadline-dev \
		libsqlite3-dev uuid-dev tk-dev

ARG PYTHON_VERSION=3.12.0
ENV PYTHON_VERSION ${PYTHON_VERSION}

ENV SRCDIR /python
RUN set -xe ; \
	mkdir -p ${SRCDIR}/build ; \
	curl -sL "https://www.python.org/ftp/python/${PYTHON_VERSION}/Python-${PYTHON_VERSION}.tar.xz" | \
	tar -xJC ${SRCDIR} --strip-components=1 -f -

WORKDIR ${SRCDIR}/build

RUN set -xe ; \
	gnuArch="$(dpkg-architecture --query DEB_BUILD_GNU_TYPE)" ;  \
	../configure \
	--build="$gnuArch" \
	--enable-shared

RUN set -xe ; \
	make -j $(( 1 * $( egrep '^processor[[:space:]]+:' /proc/cpuinfo | wc -l ) )) ; \
	make install ; \
	ldconfig

FROM python-dev as base

RUN set -xe ; \
	find /usr/local -type f -name "*.so" -exec strip --strip-unneeded {} + ; \
	find /usr/local -type f -name "*.so" | ldconfig ; \
	find /usr/local -depth \
	\( \
		\( -type d -a \( -name test -o -name tests \) \) \
			-o \
		\( -type f -a \( -name '*.pyo' -o -name '*.exe' \) \) \
	\) -exec rm -rf '{}' + ; \
	python3 -m compileall -q /usr/local/lib/python*/

RUN set -xe ; \
	rm -fr /usr/local/lib/python*/config-* ; \
	rm -rf /usr/local/lib/python*/ensurepip ; \
	rm -rf /usr/local/lib/python*/site-packages/pip* ; \
	rm -rf /usr/local/lib/python*/site-packages/setuptools* ; \
	rm -rf /usr/local/lib/python*/idlelib ; \
	rm -rf /usr/local/lib/python*/tkinter ; \
	rm -rf /usr/local/lib/python*/turtledemo ; \
	rm -rf /usr/local/lib/python*/turtle.py ; \
	rm -rf /usr/local/lib/python*/pydoc.py ; \
	rm -rf /usr/local/lib/python*/pydoc_data ; \
	rm -rf /usr/local/lib/python*/doctest.py ; \
	rm -rf /usr/local/lib/python*/unittest ; \
	rm -rf /usr/local/lib/python*/lib2to3 ; \
	rm -rf /usr/local/lib/python*/distutils ; \
	rm -rf /usr/local/lib/python*/venv ; \
	rm -rf /usr/local/lib/python*/curses ; \
	rm -rf /usr/local/lib/python*/sqlite3 ; \
	rm -rf /usr/local/lib/python*/multiprocessing ; \
	rm -rf /usr/local/lib/python*/xmlrpc ; \
	rm -rf /usr/local/lib/python*/dbm ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_test* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_codecs_jp* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_codecs_hk* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_codecs_cn* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_codecs_kr* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_codecs_tw* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_codecs_iso* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_multiprocess* ; \
	rm -rf /usr/local/lib/python*/lib-dynload/_sqlite* ; \
	rm -rf /usr/local/bin/idle3* ; \
	rm -rf /usr/local/bin/2to3* ; \
	rm -rf /usr/local/bin/pip* ; \
	rm -rf /usr/local/bin/pydoc* ; \
	strip -s /usr/local/lib/libpython*.so.* ; \
	strip -s /usr/local/bin/python3.* 2>/dev/null || true

FROM scratch

COPY --from=base /usr/local/bin/ /usr/local/bin/
COPY --from=base /usr/local/lib/ /usr/local/lib/
COPY --from=base /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=base /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6
COPY --from=base /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=base /etc/ld.so.cache /etc/ld.so.cache

# Shared libs that Python's C-extension modules link against. Keeping
# them around means the trimmed interpreter can still load things like
# binascii / zlib (libz), hashlib (libssl/libcrypto), ssl, xml (libexpat),
# _bz2, _lzma, and ctypes (libffi) — covering most pip packages that
# don't ship their own C binary.
COPY --from=base /lib/x86_64-linux-gnu/libz.so.1      /lib/x86_64-linux-gnu/libz.so.1
COPY --from=base /lib/x86_64-linux-gnu/libexpat.so.1  /lib/x86_64-linux-gnu/libexpat.so.1
COPY --from=base /lib/x86_64-linux-gnu/libbz2.so.1.0  /lib/x86_64-linux-gnu/libbz2.so.1.0
COPY --from=base /lib/x86_64-linux-gnu/liblzma.so.5   /lib/x86_64-linux-gnu/liblzma.so.5
COPY --from=base /lib/x86_64-linux-gnu/libffi.so.8    /lib/x86_64-linux-gnu/libffi.so.8
COPY --from=base /lib/x86_64-linux-gnu/libssl.so.3    /lib/x86_64-linux-gnu/libssl.so.3
COPY --from=base /lib/x86_64-linux-gnu/libcrypto.so.3 /lib/x86_64-linux-gnu/libcrypto.so.3
COPY --from=base /lib/x86_64-linux-gnu/libgcc_s.so.1  /lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=base /lib/x86_64-linux-gnu/libstdc++.so.6 /lib/x86_64-linux-gnu/libstdc++.so.6
COPY --from=base /lib/x86_64-linux-gnu/librt.so.1     /lib/x86_64-linux-gnu/librt.so.1
COPY --from=base /lib/x86_64-linux-gnu/libpthread.so.0 /lib/x86_64-linux-gnu/libpthread.so.0
COPY --from=base /lib/x86_64-linux-gnu/libdl.so.2     /lib/x86_64-linux-gnu/libdl.so.2
