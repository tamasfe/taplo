#!/bin/sh
cargo run --manifest-path ./util/schema-index/Cargo.toml -- -o site/static/schema_index.json --url https://taplo.tamasfe.dev/schemas ./schemas || exit 1
rm -rf site/static/schemas || exit 1
cp -r schemas site/static || exit 1