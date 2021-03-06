#!/bin/sh

set -e

TARGET=target/armv7-unknown-linux-gnueabihf/release
WQMS=$TARGET/wmnet
WQMS_BOT=$TARGET/wmnet-bot
WQMS_COLLECT=$TARGET/wmnet-collect
WQMS_INKY=$TARGET/wmnet-inkyphat

#cargo build --target=arm-unknown-linux-gnueabi --release
cross build --target=armv7-unknown-linux-gnueabihf --release
arm-linux-gnueabi-strip "$WQMS"
arm-linux-gnueabi-strip "$WQMS_BOT"
arm-linux-gnueabi-strip "$WQMS_COLLECT"
arm-linux-gnueabi-strip "$WQMS_INKY"