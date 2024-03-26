# Install a Display Manager (micro tut)

## Install display manager

_LXDM - MDM - SDDM - XDM - GDM - SDDM_

```bash
sudo pacman -s lxdm
```

## Stop and disable current display manager

```bash
sudo systemctl stop gdm.service
sudo systemctl disable gdm.service
```

## Start the display manager you chose

```bash
sudo systemctl start lxdm.service
sudo systemctl enable lxdm.service
```

## reboot

`sudo reboot`
