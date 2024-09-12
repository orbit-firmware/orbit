#!/bin/bash

RESET='\033[0m'
RED='\033[0;31m'
GREEN='\033[0;32m'

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
ROOT=$DIR/..

KEYBOARD=$1

if [ -z "$KEYBOARD" ]; then
    echo -e "${RED}Usage: $0 <keyboard>${RESET}"
    exit 1
fi

KEYBOARD_CONFIG=$ROOT/rmk/keyboards/$KEYBOARD.toml
if [ ! -f "$KEYBOARD_CONFIG" ]; then
    echo -e "${RED}Keyboard $KEYBOARD not found!${RESET}"
    exit 1
fi

CHIP=$(grep "chip = " $KEYBOARD_CONFIG | cut -d '"' -f 2)
CHIP_DIR=$ROOT/rmk/chips/$CHIP

if [ ! -d "$CHIP_DIR" ]; then
    echo -e "${RED}Chip $CHIP not found!${RESET}"
    exit 1
fi

cd $CHIP_DIR

TARGET=$(grep "\[target." $CHIP_DIR/.cargo/config.toml | cut -d '.' -f 2)
TARGET=${TARGET%?}

rustup component add llvm-tools-preview
rustup target add $TARGET

echo -e "${GREEN}Compiling keyboard $KEYBOARD with chip $CHIP.${RESET}"

# build
RMK_KEYBOARD=$KEYBOARD cargo run --target $TARGET --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Compilation failed!${RESET}"
    exit 1
fi
