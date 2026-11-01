# The guest rootfs for {{name}}: the published node base plus the packages
# package.json names. `hluk build` builds it and exports it as
# .hluk/rootfs.cpio; `hluk run` boots it.
#
# The guest runs the script with cwd = /, so require() resolves from
# /node_modules (Node's search walks up from the module's directory).
FROM node:22-alpine@sha256:fe4f6d74e20370acb04ad889077d14d0f64a9c8ea6a42bc60cddf1e844699914 AS deps
WORKDIR /nm
COPY package.json .
RUN npm install --omit=dev --no-audit --no-fund

FROM {{base}}
COPY --from=deps /nm/node_modules /node_modules
