# About

Initscripts rename_device binary rewritten using rust.

## How to use it

This repository provides source code for rename_device binary. The binary requires env INTERFACE to be set.

```
INTERFACE=eth0 cargo run --release
```

Environment variable ``INTERFACE`` taks name of the interface. The binary also supports flags like ``--help`` and ``--version``.
