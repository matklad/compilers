#!/bin/sh

set -e
cargo build --package runtime --release
mkdir -p target/asm
nasm -felf64 pyt/hello.asm -o target/asm/hello.o
ld -lc target/asm/hello.o target/release/libruntime.a -o target/asm/hello