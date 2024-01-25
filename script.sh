#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

dynamic_only=${ONLY_DYNAMIC_SYMBOLS:-0}

rm -rf build target
mkdir build

for f in ext_issue110624 ext_issue110624_2 ext_rfc3556; do
    gcc -c -o build/$f.o $f.c
    ar r build/lib$f.a build/$f.o
done

cargo build
#cargo rustc --
#cargo rustc -- -C save-temps=yes --print link-args
#cargo rustc -- -l static=ext_issue110624


echo '=== DYNAMIC SYMBOLS ==='
nm -D target/debug/libx.so | grep 'foo\|bar'
if [ $dynamic_only = 0 ]; then
    echo '=== NORMAL SYMBOLS ==='
    nm target/debug/libx.so | grep 'foo\|bar'
fi
