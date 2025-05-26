# syntax=docker/dockerfile:1

ARG RUST_VERSION=1
ARG XX_VERSION=latest

FROM --platform=$BUILDPLATFORM tonistiigi/xx:${XX_VERSION} AS xx
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine AS build-base
COPY --from=xx / /

SHELL [ "/bin/ash", "-c" ]

RUN \
    --mount=type=cache,id=apk,target=/var/cache/apk \
    --mount=type=cache,id=apk,target=/etc/apk/cache \
    <<EOF
apk upgrade -Ua
apk add --no-cache \
  bash clang lld llvm file git pkgconf build-base maturin
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
RUN \
    --mount=type=cache,id=apk-cross,sharing=private,target=/var/cache/apk \
    --mount=type=cache,id=apk-cross,sharing=private,target=/etc/apk/cache \
    <<EOF
#!/usr/bin/env bash
set -euxo pipefail

xx-apk add \
  "xx-cxx-essentials"
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

cargo_target_env=$(xx-cargo --print-target-triple | tr '[:lower:]' '[:upper:]' | tr - _)

export "CARGO_TARGET_${cargo_target_env}_CC=$(xx-cargo --print-target-triple)-clang"
export "CARGO_TARGET_${cargo_target_env}_LINKER=$(xx-cargo --print-target-triple)-clang"
export "CARGO_TARGET_${cargo_target_env}_RUSTFLAGS"="-C link-arg=-fuse-ld=lld -C target-feature=+crt-static"

export "TARGET_CC=$(xx-clang --print-target-triple)-clang"
export "TARGET_LINKER=$(xx-clang --print-target-triple)-clang"

export PKG_CONFIG="$(xx-cargo --print-target-triple)-pkg-config"

cargo build \
  --verbose \
  --frozen \
  --release \
  --target "$(xx-cargo --print-target-triple)" \
  --package taplo-cli

maturin build \
  --verbose \
  --frozen \
  --release \
  --target "$(xx-cargo --print-target-triple)" \
  --out /maturin

if ! xx-info is-cross; then
  maturin sdist \
    --verbose \
    --out /maturin
fi

xx-verify "${CARGO_TARGET_DIR}"/"$(xx-cargo --print-target-triple)"/release/taplo

cp -v "${CARGO_TARGET_DIR}"/"$(xx-cargo --print-target-triple)"/release/taplo /usr/local/bin/taplo

EOF

FROM scratch AS binary
COPY --from=build /usr/local/bin/taplo .
COPY --from=build /maturin .

FROM scratch AS oci
COPY --from=build /usr/local/bin/taplo /bin/taplo
ENTRYPOINT [ "/bin/taplo" ]

FROM alpine:3 AS shell
RUN apk add -U busybox-static

FROM oci AS oci-shell
ENV PATH=/bin
COPY --from=shell /bin/busybox.static /bin/busybox
SHELL [ "/bin/busybox", "ash", "-c" ]
RUN /bin/busybox --install -s /bin
