#!/bin/sh
set -xue

# by default, the build folder is located in /tmp, which is a tmpfs. The target/ folder
# can become quite large, causing the build to fail if we don't have enough RAM.
export TMPDIR="$HOME/tmp/repro-test"
mkdir -p "$TMPDIR"

reprotest -vv --vary=-time,-domain_host --source-pattern 'Cargo.* src/' '
    RUSTC_BOOTSTRAP=1 CARGO_HOME="$PWD/.cargo" RUSTUP_HOME='"$HOME/.rustup"' \
        RUSTFLAGS="-Zremap-path-prefix-from=$HOME -Zremap-path-prefix-to=/remap-home -Zremap-path-prefix-from=$PWD -Zremap-path-prefix-to=/remap-pwd" \
        cargo build --release --verbose' \
    'target/release/tr1pd target/release/tr1pctl'
