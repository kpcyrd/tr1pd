#!/bin/sh
set -ex

case "$TARGET" in
    x86_64-unknown-linux-gnu)
        cargo build --verbose --all
        cargo test --verbose --all
        ;;
    aarch64-unknown-linux-gnu)
        export RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc-6"
        cargo build --verbose --all --target="$TARGET"
        ;;
    i686-unknown-linux-gnu)
        cargo build --verbose --all --target="$TARGET"
        ;;
esac
