#!/bin/sh

# DHCP
#sudo apt-get install isc-dhcp-server
#sudo ip addr del 10.10.1.1/24 dev eth0
#sudo ip link set dev eth0 up
#sudo systemctl start isc-dhcp-server

# GPIO 
sudo chown root.gpio /dev/gpiomem
sudo chmod g+rw /dev/gpiomem


## Service
# sudo systemctl daemon-reload
sudo systemctl enable --now ngrok.service
sudo systemctl enable --now wmnet.service
sudo systemctl enable --now wmnet-bot.service
sudo systemctl enable --now wmnet-inky.service
sudo systemctl enable --now wmnet-collect.service

# sudo systemctl start ngrok.service
sudo systemctl start wmnet.service
sudo systemctl start wmnet-bot.service
sudo systemctl start wmnet-inky.service
sudo systemctl start wmnet-collect.service

