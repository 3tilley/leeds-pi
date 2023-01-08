#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly OLD_PI=arm-unknown-linux-gnueabihf
readonly NEW_PI=armv7-unknown-linux-gnueabihf
readonly NEW_PI_64=aarch64-unknown-linux-gnu

readonly TARGET_HOST=pi@rpi1-v12b
readonly TARGET_PATH=/home/pi/leeds-pi/leeds-pi
#readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly TARGET_ARCH=$NEW_PI_64
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/leeds-pi

cross build --release --target=${TARGET_ARCH}
#rsync -e "ssh -i ../leedspi_rsa" ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
# Not launching automatically for now
#ssh -t ${TARGET_HOST} ${TARGET_PATH}
