#!/bin/sh
set -ex

case "$BUILD_MODE" in
    *)
        docker build --build-arg TARGET="$TARGET" -t "tr1pd-test-$TARGET" -f ci/Dockerfile .
        docker run -t -e TARGET="$TARGET" "tr1pd-test-$TARGET" ci/build.sh
        ;;
esac
