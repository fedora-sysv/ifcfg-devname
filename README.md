# ifcfg-devname

[![Crates.io][crates-status]][crates] [![Build and Coverage][build-status]][build] [![Lint Code Base][lint-status]][lint] [![codecov][coverage-status]][coverage] [![Mergify Status][mergify-status]][mergify]

[crates]: https://crates.io/crates/ifcfg-devname
[crates-status]: https://img.shields.io/crates/v/ifcfg-devname.svg

[build]: https://github.com/fedora-sysv/ifcfg-devname/actions/workflows/test.yml
[build-status]: https://github.com/fedora-sysv/ifcfg-devname/actions/workflows/test.yml/badge.svg

[lint]: https://github.com/fedora-sysv/ifcfg-devname/actions/workflows/linter.yml
[lint-status]: https://github.com/fedora-sysv/ifcfg-devname/actions/workflows/linter.yml/badge.svg

[coverage]: https://codecov.io/gh/fedora-sysv/ifcfg-devname
[coverage-status]: https://codecov.io/gh/fedora-sysv/ifcfg-devname/branch/main/graph/badge.svg

[mergify]: https://mergify.io
[mergify-status]: https://img.shields.io/endpoint.svg?url=https://dashboard.mergify.io/badges/fedora-sysv/ifcfg-devname&style=flat

Initscripts `rename_device` binary rewritten using rust and renamed to `ifcfg-devname`.

Program `ifcfg-devname` reads ENV **INTERFACE**, which is expected to contain the name of the network interface. Then it looks for the hardware address of such an interface. After that it scans ifcfg configuration files in directory `/etc/sysconfig/network-scripts/` and looks for configuration with **HWADDR** set to given hw address. If the program successfully finds such a configuration, it returns on standard output content of property **DEVICE** from matching ifcfg configuration. In all other cases it returns error code `1`.

## How to use it

This repository provides source code for `ifcfg-devname` binary. The binary requires env **INTERFACE** to be set.

```sh
INTERFACE=eth0 cargo run --release
```

Environment variable **INTERFACE** takes name of the interface.
