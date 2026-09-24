# The guest rootfs for {{name}}: the published node base plus the packages
# package.json names. `hluk build` builds it and exports it as
# .hluk/rootfs.cpio; `hluk run` boots it.
#
# The guest runs the script with cwd = /, so require() resolves from
# /node_modules (Node's search walks up from the module's directory).
FROM node:22-alpine AS deps
WORKDIR /nm
COPY package.json .
RUN npm install --omit=dev --no-audit --no-fund

FROM {{base}}
COPY --from=deps /nm/node_modules /node_modules
