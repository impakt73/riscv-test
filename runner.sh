#!/bin/bash

set -e

# Cargo gives us a relative path to the binary.
# We save it here as an abs path so that we can reference it after moving to
# the devsim dir.
elf_path="$(realpath $1)"

pushd ../devsim/
cargo run --release \
    --bin view       \
    -- $elf_path

# WA: devsim doesn't have an image output option, so it dumps it to $PWD.
img_path="$(realpath "./image.png")"
popd

cp -v $img_path .
