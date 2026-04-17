#!/usr/bin/env bash

set -e

# deps-bundle.sh — pack a redistributable bundle of build dependencies.
#
# After running deps.sh (which installs protoc into ./opt), run this
# script to create deps-bundle.tar.zst for CI caching.

cd -- "$( dirname -- "${BASH_SOURCE[0]}" )"

rm -f deps-bundle.tar.zst
tar -Izstd -cf deps-bundle.tar.zst \
    opt/bin opt/lib opt/include

echo "[+] Created deps-bundle.tar.zst"
