# Express HTTP server rootfs — extends the node rootfs with the express package.
#
# Build:  just build-rootfs http-express examples/http-server/express/Dockerfile
# Run:    hluk run --initrd build-elfloader/http-express-rootfs.cpio \
#             --scratch-mb 128 --net --port 8080 examples/http-server/express/server.js
#
# The guest runs the script with cwd = /, so require('express') resolves from
# /node_modules (Node's node_modules search walks up from the module directory).
FROM node:21-alpine AS deps
WORKDIR /nm
RUN npm install --no-audit --no-fund express@4

FROM hluk-node-rootfs:latest
COPY --from=deps /nm/node_modules /node_modules
