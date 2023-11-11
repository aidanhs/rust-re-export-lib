#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset

rm -rf build target
mkdir build

gcc -c -o build/ext.o ext.c
ar r build/libext.a build/ext.o

cargo build

nm target/debug/libx.so | grep 'foo\|bar'
