#!/bin/sh
# Use RELEASE=true for release builds
(cd node/lib && yarn && yarn build)