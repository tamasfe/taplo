#!/bin/sh
# Use RELEASE=true for release builds
(cd js/lsp && yarn && yarn build)