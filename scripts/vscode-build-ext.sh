#!/bin/sh
(cd taplo-ide && wasm-pack build --target nodejs) || exit
(cd node/vscode && npm i && vsce package)