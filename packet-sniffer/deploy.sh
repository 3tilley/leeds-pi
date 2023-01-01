#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@rpi12b
readonly TARGET_PATH=/home/pi/leeds-pi/leeds-pi
#readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly TARGET_ARCH=arm-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/packer-sniffer

cross build --release --target=${TARGET_ARCH}
#rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
#ssh -t ${TARGET_HOST} ${TARGET_PATH}
