#!/bin/sh
set -ex

rustup install "stable-$TARGET"
rustup target add "$TARGET" || true

apt-get update -q
apt-get install -qy libsodium-dev
