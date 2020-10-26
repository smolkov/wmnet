#!/bin/sh
sudo systemctl stop wqms-inky
sudo systemctl stop wqms-collect
sudo systemctl stop wqms-bot
sudo systemctl stop wqms

sudo systemctl disable wqms-inky
sudo systemctl disable wqms-collect
sudo systemctl disable wqms-bot
sudo systemctl disable wqms
rm -rf /home/pi/wqms
rm -rf /home/pi/wqms-bot
rm -rf /home/pi/wqms-collect
rm -rf /home/pi/wqms-inky

sudo rm -rf rm /etc/systemd/system/wqms
sudo rm -rf rm /etc/systemd/system/wqms-collect
sudo rm -rf rm /etc/systemd/system/wqms-inky
sudo rm -rf rm /etc/systemd/system/wqms-bot

