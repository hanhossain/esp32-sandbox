#!/bin/zsh

set -e

#cargo xbuild

# change this for release flashes
BIN_PATH=target/xtensa-esp32-none-elf/debug/esp32-sandbox

test -f $BIN_PATH.bin && rm $BIN_PATH.bin

# convert to bin
esptool.py --chip esp32 elf2image --flash_mode="dio" --flash_freq "40m" --flash_size "4MB" -o $BIN_PATH.bin $BIN_PATH

# flash
esptool.py --chip esp32 --port /dev/cu.SLAB_USBtoUART --baud 115200 --before default_reset --after hard_reset write_flash -z --flash_mode dio --flash_freq 40m --flash_size detect 0x1000 $BIN_PATH.bin
