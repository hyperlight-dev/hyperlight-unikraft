# Flask HTTP server rootfs — extends the slim agent rootfs with Flask.
#
# Build:  just build-rootfs http-flask examples/http-server/flask/Dockerfile
# Run:    hluk run --initrd build-elfloader/http-flask-rootfs.cpio \
#             --scratch-mb 256 --net --port 8080 examples/http-server/flask/server.py
FROM hluk-python-shell-rootfs:latest AS base

# pip runs in a real Linux stage, then the packages are copied into the rootfs.
FROM python:3.12-slim-bookworm AS installer
COPY --from=base / /rootfs/
RUN pip install --no-cache-dir --target /rootfs/usr/local/lib/python3.12/site-packages \
        flask

FROM scratch
COPY --from=installer /rootfs/ /
