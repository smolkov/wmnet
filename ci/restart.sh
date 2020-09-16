#!/bin/sh

sudo systemctl stop wqms-inky
sudo systemctl stop wqms-collect
sudo systemctl stop wqms-bot
# sudo systemctl stop ngrok

sudo systemctl start wqms-inky
sudo systemctl start wqms-collect
sudo systemctl start wqms-bot
# sudo systemctl start ngrok

