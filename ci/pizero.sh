#!/bin/sh

set -e

ARCH=arm-unknown-linux-gnueabihf

TARGET=target/$ARCH/release
WMNET=$TARGET/wmnet
WMNET_BOT=$TARGET/wmnet-bot
WMNET_COLLECT=$TARGET/wmnet-collect
WMNET_INKY=$TARGET/wmnet-inky

#cargo build --target=arm-unknown-linux-gnueabi --release
cross build --target=$ARCH --release
arm-linux-gnueabi-strip "$WMNET"
arm-linux-gnueabi-strip "$WMNET_BOT"
arm-linux-gnueabi-strip "$WMNET_COLLECT"
arm-linux-gnueabi-strip "$WMNET_INKY"