#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cd $DIR/../rmk && RMK_KEYBOARD=$1 cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed"
    exit 1
fi
cd $DIR/../rmk && RMK_KEYBOARD=$1 cargo objcopy --release -- -O binary firmware.bin
cd $DIR/../rmk && RMK_KEYBOARD=$1 cargo objcopy --release -- -O binary firmware.hex
