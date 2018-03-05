#!/bin/sh
set -ex

case "$TARGET" in
    x86_64-unknown-linux-gnu)
        cargo build --verbose --all
        cargo test --verbose --all
        ;;
    aarch64-unknown-linux-gnu)
        export RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc-6 -C ar=aarch64-linux-gnu-gcc-ar-6"
        export CC="aarch64-linux-gnu-gcc-6"
        export PKG_CONFIG="aarch64-linux-gnu-pkg-config"
        export PKG_CONFIG_ALLOW_CROSS=1
        cargo build --verbose --all --target="$TARGET"
        ;;
    i686-unknown-linux-gnu)
        export PKG_CONFIG="i686-linux-gnu-pkg-config"
        export PKG_CONFIG_ALLOW_CROSS=1
        cargo build --verbose --all --target="$TARGET"
        ;;
esac
