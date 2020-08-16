#!/bin/bash

# port esp32 is connected to (leave empty to autodetect)
PORT=/dev/cu.SLAB_USBtoUART

# baud rate for programming
FLASH_BAUDRATE=115200

# baud rate for terminal
TERM_BAUDRATE=115200

# Flash Mode
FLASH_MODE="dio"

# Flash Speed
FLASH_SPEED="40m"

# Use bootloader (needed for using irom, drom and psram)
USE_BOOTLOADER=1

# debug or release build
TYPE=debug

# address of partition table
PARTION_ADDR=0x8000

# address of application
APP_ADDR=0x10000

# skip flashing the esp
SKIPFLASH=0

#source of the utility to generate the binary partition table
GENPART_SOURCE=https://github.com/espressif/esp-idf/blob/v4.0/components/partition_table/gen_esp32part.py?raw=true

# source of the bootloader
# customized bootloader which initializes the external psram
BOOTLOADER_SOURCE=https://github.com/arjanmels/esp32_bootloader_init_extram/blob/v1.0/build/bootloader/bootloader.bin?raw=true
# default bootloader from the espressif arduino github
#BOOTLOADER_SOURCE=https://github.com/espressif/arduino-esp32/blob/idf-release/v4.0/tools/sdk/bin/bootloader_dio_40m.bin?raw=true

# color codes
STAGE="\033[1;36m"
SUCCESS="\033[1;32m"
ERROR="\033[1;31m"
RESET="\033[0m"


BIN_PATH=target/xtensa-esp32-none-elf/debug/esp32-sandbox

rm target/current.elf 2> /dev/null
rm target/current.bin 2> /dev/null

# get error code of any step of the pipe
set -o pipefail


if [[ ! -f $BIN_PATH ]]
then
    printf "${ERROR}Error: Output file ($BIN_PATH) not generated!${RESET}\n\n"
    exit 1
fi

#display section sizes
echo
xtensa-esp32-elf-readelf $BIN_PATH -S|egrep 'BIT|\[Nr\]' |awk 'BEGIN {FS="[ \t\[\]]+"}  $9~/A|Flg/ {size=sprintf("%7d", "0x" $7)+0; printf("%-3s %-20s %-8s %-8s %-8s %8s %-3s %-3s\n",$2,$3,$4,$5,$7,((size>0)?size:$7),$9,$12); total+=size; } END { printf("\nTotal: %d bytes\n",total)}'
echo

# convert to bin
rm $BIN_PATH.bin 2>/dev/null
esptool.py --chip esp32 elf2image --flash_mode=$FLASH_MODE --flash_freq $FLASH_SPEED  -o $BIN_PATH.bin $BIN_PATH > /dev/null
if [ $? -ne 0 ]
then
    printf "${ERROR}Error: Output file ($BIN_PATH).bin not generated!${RESET}\n\n"
    esptool.py --chip esp32 elf2image --flash_mode=$FLASH_MODE --flash_freq $FLASH_SPEED  -o $BIN_PATH.bin $BIN_PATH
    exit 1
fi

esptool.py --chip esp32 image_info $BIN_PATH.bin |egrep -v -i "esptool.py|Image version|Checksum|Validation Hash|Segments"
echo

if [ $? -ne 0 ]
then
    exit 1
fi

# create links to output binaries for linking with debugger
ln -sf $(pwd)/$BIN_PATH target/current.elf
ln -sf $(pwd)/$BIN_PATH.bin target/current.bin

if [[ $SKIPFLASH -ne 1 ]]
then
    flash() {
        echo -e "${STAGE}Flashing...${RESET} $@\n"
        esptool.py --chip esp32 --port $PORT --baud $FLASH_BAUDRATE --after hard_reset write_flash --flash_mode=$FLASH_MODE --flash_freq $FLASH_SPEED --flash_size detect ${@} |egrep -v -i "stub|baud rate|Changed.|Configuring flash size|Serial port|esptool.py|Leaving"
    }

    if [[ !USE_BOOTLOADER -eq 1 ]]
    then
        flash 0x1000 $BIN_PATH.bin 
    else

        printf "${STAGE}Creating partition table... ${RESET}"
        if [[ target/partitions.bin -ot partitions.csv ]]
        then
            printf "\n\n"
            # get gen_esp32part.py and create binary partition table
            curl -s -S -L $GENPART_SOURCE --output target/gen_esp32part.py
            
            rm target/partitions.bin 2> /dev/null
            python target/gen_esp32part.py partitions.csv target/partitions.bin
    
            echo
        else
            printf "skipping as it is up to date\n\n"
        fi


        # get bootloader.bin file 
        # (different variants exist, but only difference is flash settings which are overriden by esptool)
        curl -s -S -L $BOOTLOADER_SOURCE --output target/bootloader.bin

        # check if bootloader.bin and paritions.bin are already correctly flashed (to prevent unnecessary writes)
        printf "${STAGE}Verify bootloader and partition table...${RESET} "
        esptool.py --no-stub --chip esp32 --port $PORT --baud $FLASH_BAUDRATE verify_flash 0x1000 target/bootloader.bin $PARTION_ADDR target/partitions.bin > /dev/null
        if [ $? -ne 0 ]; then
            printf "modified\n\n"
            # flash bootloader.bin, partitions.bin and application
            flash 0x1000 target/bootloader.bin $PARTION_ADDR target/partitions.bin $APP_ADDR $BIN_PATH.bin
        else
            printf "unchanged\n\n"
            # flash application only
            flash $APP_ADDR $BIN_PATH.bin  
        fi
    fi

    if [[ $? -ne 0 ]]
    then
        printf "\n${ERROR}Error flashing application${RESET}\n\n"
        exit 1
    fi
fi


if [[ $SKIPFLASH -ne 1 ]]
then
    printf "\n${SUCCESS}Succesfully build and flashed application${RESET}\n\n"
else
    printf "\n${SUCCESS}Succesfully build application${RESET}\n\n"
fi
