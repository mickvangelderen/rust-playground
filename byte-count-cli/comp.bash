#!/usr/bin/env bash

mkdir -p build

rustc -C opt-level=3 bin/threaded.rs -o build/rust-multi
rustc -C opt-level=3 src/main.rs -o build/rust-single
# rustc -C opt-level=3 src/main-bytes.rs -o build/rust-unbuf-single
g++ cpp/main.cpp -O3 -o build/cpp-single
# gcc c/main.c -O3 -o build/c-unbuf-single
gcc c/main-buffered.c -O3 -o build/c-single
