#!/bin/bash -ex

BASEDIR=$(realpath "$(dirname "$0")")

for test_dir in $BASEDIR/*; do
    if [ ! -d "$test_dir" ]; then
        continue
    fi

    pushd "$test_dir"

    RUST_TARGET_PATH="$BASEDIR/.." \
        cargo xbuild --target x86_64-linux-kernel-module
    make -C "$BASEDIR" M="$test_dir"
    rustc --tests "$test_dir/tests.rs"
    # TODO: qemu stuff!
    ./lib

    popd
done
