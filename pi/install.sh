#!/usr/bin/env bash
set -xeu # print command, exit on error, undefined variables as error

if ! grep 'dtoverlay=dwc2' /boot/config.txt; then echo "dtoverlay=dwc2" >> /boot/config.txt; fi
if ! grep dwc2 /etc/modules; then echo "dwc2" >> /etc/modules; fi

cp ./enable-hid /opt/enable-hid
chmod u+x /opt/enable-hid
cp ./enable-hid.service /lib/systemd/system/enable-hid.service
cp ./hatel.service /lib/systemd/system/hatel.service

systemctl daemon-reload
systemctl enable enable-hid.service
systemctl enable hatel.service
systemctl start hatel.service
