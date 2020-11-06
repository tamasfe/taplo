#!/bin/sh
# Use RELEASE=true for release builds
(cd js/cli && yarn && yarn build)