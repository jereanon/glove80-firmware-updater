![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# glove80-firmware-updater


A simple command line utility to update the firmware on a [Glove80]((https://www.moergo.com/) device.

## Examples

Run the firmware updater with default values:
```bash
glove80-firmware-updater -f firmware.uf2
```
Run the firmware updater with a full path to the firmware file:
```bash
glove80-firmware-updater -f /home/user/firmware.uf2
```

Run the firmware updater with non-default values:
```bash
glove80-firmware-updater -f firmware.uf2 -l GLV80LHBOOT -r GLV80RHBOOT
```


Current version: 0.1.0

License: BSD-2-Clause
