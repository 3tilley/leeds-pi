#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@rpi1-v12b
readonly TARGET_PATH=/home/pi/leeds-pi/basic-rust
#readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
#readonly TARGET_ARCH=arm-unknown-linux-musleabihf
readonly TARGET_ARCH=arm-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/basic-rust

cargo build --release --target=${TARGET_ARCH}
rsync -e "ssh -i ../leedspi_rsa" ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -i ../leedspi_rsa -t ${TARGET_HOST} ${TARGET_PATH}
