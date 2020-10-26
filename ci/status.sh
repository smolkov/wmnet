#!/bin/sh

sudo systemctl status wmnet-inky
sudo systemctl status wmnet-collect
sudo systemctl status wmnet-bot
sudo systemctl status wmnet
sudo systemctl status isc-dhcp-server

# sudo systemctl stop ngrok

# sudo systemctl start wmnet-inky
# sudo systemctl start wmnet-collect
# sudo systemctl start wmnet-bot
# sudo systemctl start wmnet
# sudo systemctl start ngrok
