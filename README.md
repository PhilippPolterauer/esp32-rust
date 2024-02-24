# ESP32-WROOM-D Rust in wsl2 + devcontainer on Windows

This is a playground for testing a esp32-wroom-d development from windows using wsl and a devcontainer.

## 
```bash
# isntall the latest espflash version
cargo install espflash@=3.0.0-rc.1 --force
```
References
- https://github.com/esp-rs/esp-idf-hal/tree/master/examples
- https://docs.espressif.com/projects/esptool/en/latest/esp32/installation.html
- https://www.espressif.com/sites/default/files/documentation/esp32-wroom-32_datasheet_en.pdf
- https://github.com/dorssel/usbipd-win

## Correct Windows Driver 

It is important to use the correct windows driver otherwise you will get an retry timeout error for the esp32.
Use `CP210x Windows Drivers` from https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers?tab=downloads
and **NOT** `CP210x Universal Windows Driver`

## Forwarding the device to WSL2

the easiest solution I found,  which worked out of the box is 
`usbipd-win` from https://github.com/dorssel/usbipd-win. After downloading and installing it you can follow the following steps.

1) checking for the adapter

```powershell
PS C:\Users\phili> usbipd list
Connected:
BUSID  VID:PID    DEVICE                                                        STATE
1-12   1a86:7523  Silicon Labs CP210x USB to UART Bridge (COM6)                 Not shared
5-2    0bda:8153  Realtek USB GbE Family Controller                             Not shared
```

2) binding the device

the busid in my case is `1-12` then start a administrator terminal and run 
```ps
usbipd bind -b 1-12
```

this leads to the following 
```powershell
PS C:\Users\phili> usbipd list
Connected:
BUSID  VID:PID    DEVICE                                                        STATE
1-12   1a86:7523  Silicon Labs CP210x USB to UART Bridge (COM6)                 Shared
```

after this run `usbipd attach --wsl -b 1-12`
    
> [!NOTE] 
> this does not need to be run in an elevated shell, but it needs to be ran each time the usb cable is disconnected from the esp32.

3) Attaching the device to wsl
```powershell
PS C:\Users\phili> usbipd attach --wsl -b 1-12
usbipd: info: Using WSL distribution 'Ubuntu' to attach; the device will be available in all WSL 2 distributions.
usbipd: info: Using IP address 172.24.96.1 to reach the host.
```

the device should be visible from within `wsl`.

4) checking for the device in wsl
```bash
# wsl Ubuntu 22.04
philipp@Philipp:~$ ls -al /dev/ttyUSB0
crw-rw---- 1 root dialout 188, 0 Feb 24 12:36 /dev/ttyUSB0
```

> [!WARNING]
> it is important that your user is inside the dialout group otherwise you are not allowed to use the `/dev/ttyUSB0` device. You can verify if you are in the group by 
> ```console
> $ groups | grep dialout
> philipp adm dialout cdrom floppy sudo audio dip video plugdev netdev docker
> ```
> if you are not in the group run `sudo usermod -aG dialout $USER`

5) checking for esp `espflash board-info`: espflash installed e.g. via `cargo install espflash@3.0.0-rc.1`

```console 
philipp@Philipp:~$ espflash board-info
[2024-02-24T11:46:35Z INFO ] Serial port: '/dev/ttyUSB0'
[2024-02-24T11:46:35Z INFO ] Connecting...
[2024-02-24T11:46:35Z INFO ] Using flash stub
Chip type:         esp32 (revision v3.1)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       e4:65:b8:77:6e:78
```

