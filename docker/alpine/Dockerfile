# syntax=docker/dockerfile:1

ARG DISTRIBUTION_VERSION
ARG RUST_VERSION
ARG XX_VERSION=latest

FROM --platform=$BUILDPLATFORM tonistiigi/xx:${XX_VERSION} AS xx
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine${DISTRIBUTION_VERSION} AS build-base
COPY --from=xx / /

SHELL [ "/bin/ash", "-c" ]

# install host dependencies
ARG DISTRIBUTION_PACKAGES
RUN \
  --mount=type=cache,id=apk,sharing=private,target=/var/cache/apk \
  --mount=type=cache,id=apk,sharing=private,target=/etc/apk/cache \
  <<EOF

apk upgrade -Ua
apk add --no-cache \
  bash clang lld llvm file git pkgconf build-base \
  ${DISTRIBUTION_PACKAGES}
EOF

WORKDIR /source
COPY --link . .

FROM build-base AS build-prep

ENV CARGO_TARGET_DIR='/root/.cache/rust'

RUN \
  --mount=type=cache,id=crates,target=/usr/local/cargo/git/db \
  --mount=type=cache,id=crates,target=/usr/local/cargo/registry/cache \
  --mount=type=cache,id=crates,target=/usr/local/cargo/registry/index \
  <<EOF
#!/usr/bin/env bash
set -euxo pipefail

cargo fetch --locked --verbose
EOF

# Install target dependencies
ARG TARGETPLATFORM
ARG DISTRIBUTION_PACKAGES
RUN \
  --mount=type=cache,id=apk,sharing=private,target=/var/cache/apk \
  --mount=type=cache,id=apk,sharing=private,target=/etc/apk/cache \
  <<EOF
#!/usr/bin/env bash
set -euxo pipefail

xx-apk add \
  "xx-cxx-essentials" \
  ${DISTRIBUTION_PACKAGES}
EOF

FROM build-prep AS build

ENV CARGO_TARGET_DIR='/root/.cache/rust'

RUN \
  --mount=type=cache,id=crates,sharing=shared,readonly,target=/usr/local/cargo/git/db \
  --mount=type=cache,id=crates,sharing=shared,readonly,target=/usr/local/cargo/registry/cache \
  --mount=type=cache,id=crates,sharing=shared,readonly,target=/usr/local/cargo/registry/index \
  --mount=type=cache,id=build,sharing=private,target=/root/.cache \
  <<EOF
#!/usr/bin/env bash
set -euxo pipefail

xx-clang --setup-target-triple
xx-clang --wrap

if ! xx-info is-cross; then
  export OPENSSL_DIR="/usr"
else
  export OPENSSL_DIR="/$(xx-info triple)/usr"
fi

export OPENSSL_STATIC="1"
export PKG_CONFIG_ALL_STATIC="1"
export OPENSSL_NO_VENDOR="1"
export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld -C target-feature=+crt-static"
export PKG_CONFIG="$(xx-clang --print-prog-name=pkg-config)"

xx-cargo build \
  --verbose \
  --frozen \
  --bin taplo \
  --release \
  --features toml-test,lsp

xx-verify "${CARGO_TARGET_DIR}"/"$(xx-cargo --print-target-triple)"/release/taplo

mv -v "${CARGO_TARGET_DIR}"/"$(xx-cargo --print-target-triple)"/release/taplo /usr/local/bin/taplo

EOF

FROM scratch AS binary
COPY --from=build /usr/local/bin/taplo .

FROM scratch AS oci
COPY --from=build /usr/local/bin/taplo /taplo
ENTRYPOINT [ "/taplo" ]
