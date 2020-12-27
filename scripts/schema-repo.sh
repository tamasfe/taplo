#!/bin/sh
cargo run --manifest-path ./util/schema-index/Cargo.toml -- -o site/static/schema_index.json --url https://taplo.tamasfe.dev/schemas ./schemas
rm -rf site/static/schemas
cp -r schemas site/static