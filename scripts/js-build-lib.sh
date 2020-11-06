#!/bin/sh
# Use RELEASE=true for release builds
(cd js/lib && yarn && yarn build)