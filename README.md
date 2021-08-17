# About

Initscripts rename_device binary rewritten using rust.

Program rename_device reads ENV INTERFACE, which is expected to contain the name of the network interface. Then it looks for the hardware address of such an interface. After that it looks at the kernel command line for key-value-pair ifname=NEW_NAME:MAC_ADDRESS with given mac address. If a new name wasn't found and kernel cmd it scans ifcfg configuration files in directory /etc/sysconfig/network-scripts/ and looks for configuration with HWADDR set to given hw address. If the program successfully finds such a configuration, it returns on standard output content of property DEVICE from matching ifcfg configuration. In all other cases it returns error code 1.

## How to use it

This repository provides source code for rename_device binary. The binary requires env INTERFACE to be set.

```
INTERFACE=eth0 cargo run --release
```

Environment variable ``INTERFACE`` takes name of the interface.
