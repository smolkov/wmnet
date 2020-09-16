#!/bin/sh

set -e

TARGET=target/armv7-unknown-linux-gnueabihf/release
WQMS=$TARGET/wqms
WQMS_BOT=$TARGET/wqms-bot
WQMS_COLLECT=$TARGET/wqms-collect
WQMS_INKY=$TARGET/wqms-inkyphat

#cargo build --target=arm-unknown-linux-gnueabi --release
cross build --target=armv7-unknown-linux-gnueabihf --release
arm-linux-gnueabi-strip "$WQMS"
arm-linux-gnueabi-strip "$WQMS_BOT"
arm-linux-gnueabi-strip "$WQMS_COLLECT"
arm-linux-gnueabi-strip "$WQMS_INKY"