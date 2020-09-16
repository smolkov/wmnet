#!/bin/sh

set -e

ARCH=arm-unknown-linux-gnueabihf

TARGET=target/$ARCH/release
WQMS=$TARGET/wqms
WQMS_BOT=$TARGET/wqms-bot
WQMS_COLLECT=$TARGET/wqms-collect
WQMS_INKY=$TARGET/wqms-inky

#cargo build --target=arm-unknown-linux-gnueabi --release
cross build --target=$ARCH --release
# arm-linux-gnueabi-strip "$WQMS"
arm-linux-gnueabi-strip "$WQMS_BOT"
arm-linux-gnueabi-strip "$WQMS_COLLECT"
arm-linux-gnueabi-strip "$WQMS_INKY"

