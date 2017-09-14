#!/usr/bin/env bash

DATA="source-code-pro-variable-fonts.zip"

echo rust-multi
time build/rust-multi "${DATA}" > rust-multi.txt

echo rust-single
time build/rust-single "${DATA}" > rust-single.txt

# echo rust-unbuf-single
# time build/rust-unbuf-single "${DATA}" > rust-unbuf-single.txt

echo cpp-single
time build/cpp-single "${DATA}" > cpp-single.txt

# echo c-unbuf-single
# time build/c-unbuf-single "${DATA}" > c-unbuf-single.txt

echo c-single
time build/c-single "${DATA}" > c-single.txt
