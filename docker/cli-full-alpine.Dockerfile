FROM rust:1.61-alpine3.16 as build

WORKDIR /build

RUN apk add --no-cache musl-dev libressl-dev

COPY . .

RUN cargo build -p taplo-cli --release --all-features

FROM alpine:3.16

COPY --from=build /build/target/release/taplo /usr/bin/taplo

ENTRYPOINT [ "/usr/bin/taplo" ]
