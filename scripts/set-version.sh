#!/bin/sh
set -e
P=$(echo "$1" | sed 's:/*$::')
sed -i '0,/^version\s*=\s*".*"$/s//version = "'"$2"'"/' "$P/Cargo.toml"
echo "version \"$2\" for $P has been set"
