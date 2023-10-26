FROM rust:1.73-alpine3.18 as build

WORKDIR /build

RUN apk add --no-cache musl-dev

COPY . .

RUN cargo build -p taplo-cli --release --features toml-test,lsp

FROM alpine:3.18

COPY --from=build /build/target/release/taplo /usr/bin/taplo

ENTRYPOINT [ "/usr/bin/taplo" ]
