# pi notes

https://stackoverflow.com/questions/20943322/accessing-keys-from-linux-input-device

https://eleccelerator.com/tutorial-about-usb-hid-report-descriptors/

- low power mode: https://github.com/seamusdemora/PiFormulae/blob/master/RPi4bSleep.md
- emulate keyboard with usb c https://mtlynch.io/key-mime-pi/
- enable hid: https://github.com/mtlynch/key-mime-pi/blob/master/enable-usb-hid
- pacman -S alsa-lib

rpi-eeprom-config --edit

rustup target add armv7-unknown-linux-gnueabihf

This post was a lifesaver, it went into detail specifically explaining how to get cross setup with alsa/pkg-config:

https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/

To build the arm version, make sure you have cross installed, then build the local cross docker image and run cross:

```
docker build ./pi -t cross:hatel
cross build --target armv7-unknown-linux-gnueabihf
```

# Connect to pi over ssh
```
ifconfig # Look for inet & mask
nmap -sn 192.168.0.0/24
```
