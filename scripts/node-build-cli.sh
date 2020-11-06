#!/bin/sh
# Use RELEASE=true for release builds
(cd node/cli && yarn && yarn build)