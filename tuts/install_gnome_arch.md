# Install Gnome on Arch linux (micro tut)

## Update Arch Linux & reboot

```bash
sudo pacman -Syu; sudo reboot
```

## Install xorg (if not using wayland)

use defaults

```bash
sudo pacman -S xorg xorg-server
```

## install gnome desktop environment

use defaults

```bash
sudo pacman -S gnome
```

## start and enable gdm.service

```bash
sudo systemctl start gdm.service
```

log into gnome then run:

```bash
sudo systemctl enable gdm.service
```
