#!/bin/sh

set -e
cargo build --package runtime --release
mkdir -p target/asm
nasm -felf64 pyt/hello.asm -o target/asm/hello.o
ld target/asm/hello.o -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lutil target/release/libruntime.a -o target/asm/hello