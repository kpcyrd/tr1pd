#!/bin/sh
set -ex

case "$BUILD_MODE" in
    docker)
        docker build -t tr1pd .
        docker images tr1pd
        ;;
    reprotest)
        docker build -t reprotest-tr1pd -f ci/Dockerfile.reprotest .
        docker run --privileged reprotest-tr1pd ci/reprotest.sh
        ;;
    *)
        docker build --build-arg TARGET="$TARGET" -t "tr1pd-test-$TARGET" -f ci/Dockerfile .
        docker run -t -e TARGET="$TARGET" "tr1pd-test-$TARGET" ci/build.sh
        ;;
esac
