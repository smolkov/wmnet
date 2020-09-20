#!/bin/sh

set -e
ARCH=arm-unknown-linux-gnueabihf

TARGET=target/$ARCH/release
WQMS_BOT=$TARGET/wqms-bot
WQMS_COLLECT=$TARGET/wqms-collect
WQMS_INKY=$TARGET/wqms-inky
HOST=192.168.0.5
PORT=22

rsync -av $WQMS_BOT pi@$HOST:
rsync -av $WQMS_COLLECT pi@$HOST:
rsync -av $WQMS_INKY pi@$HOST:

# rsync -azP -e "ssh -p $PORT" $WQMS pi@$HOST:
# rsync -azP -e "ssh -p $PORT" $WQMS_COLLECT pi@$HOST
# rsync -azP -e "ssh -p $PORT" $WQMS_INKY pi@$HOST