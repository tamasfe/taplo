#!/bin/sh
(cd util/cargo-schema && cargo run > ../../taplo-ide/src/schema/cargo.json)