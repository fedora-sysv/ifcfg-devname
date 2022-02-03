# ifcfg-devname

[![Crates.io](https://img.shields.io/crates/v/ifcfg-devname.svg)](https://crates.io/crates/ifcfg-devname) [![Build and Coverage](https://github.com/fedora-sysv/ifcfg-devname/actions/workflows/ifcfg-devname-test-coverage.yml/badge.svg)](https://github.com/fedora-sysv/ifcfg-devname/actions/workflows/ifcfg-devname-test-coverage.yml) [![codecov](https://codecov.io/gh/fedora-sysv/ifcfg-devname/branch/main/graph/badge.svg)](https://codecov.io/gh/fedora-sysv/ifcfg-devname) [![Mergify Status][mergify-status]][mergify]

[mergify]: https://mergify.io
[mergify-status]: https://img.shields.io/endpoint.svg?url=https://dashboard.mergify.io/badges/fedora-sysv/ifcfg-devname&style=flat

Initscripts `rename_device` binary rewritten using rust and renamed to `ifcfg-devname`.

Program `ifcfg-devname` reads ENV **INTERFACE**, which is expected to contain the name of the network interface. Then it looks for the hardware address of such an interface. After that it looks at the kernel command line for key-value-pair `ifname=NEW_NAME:MAC_ADDRESS` with given mac address. If a new name wasn't found and kernel cmdline it scans ifcfg configuration files in directory `/etc/sysconfig/network-scripts/` and looks for configuration with **HWADDR** set to given hw address. If the program successfully finds such a configuration, it returns on standard output content of property **DEVICE** from matching ifcfg configuration. In all other cases it returns error code `1`.

## How to use it

This repository provides source code for `ifcfg-devname` binary. The binary requires env **INTERFACE** to be set.

```
INTERFACE=eth0 cargo run --release
```

Environment variable **INTERFACE** takes name of the interface.

If you wish to run integration tests and unit tests, you can do so by:

```
cargo test
```
