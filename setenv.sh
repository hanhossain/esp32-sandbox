#!/bin/zsh

# change this to the directory of where you built rustc for xtensa
CUSTOM_RUSTC=/Users/hanhossain/Developer/github/MabezDev/rust-xtensa
TARGET_TRIPLE=x86_64-apple-darwin

export RUST_BACKTRACE=1
export XARGO_RUST_SRC=$CUSTOM_RUSTC/library
export RUSTC=$CUSTOM_RUSTC/build/$TARGET_TRIPLE/stage2/bin/rustc
export RUSTDOC=$CUSTOM_RUSTC/build/$TARGET_TRIPLE/stage2/bin/rustdoc
export PATH=$PATH:$HOME/esp/xtensa-esp32-elf/bin
