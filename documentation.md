---
layout: default
---

# Documentation

For guide-level documentation there is [The Drone Book](https://book.drone-os.com).

## API documentation

Drone is split into multiple crates:

**drone-core**
[0.10](https://api.drone-os.com/drone-core/0.10/)
[0.11](https://api.drone-os.com/drone-core/0.11/)
[0.12](https://api.drone-os.com/drone-core/0.12/)
[0.13 (latest)](https://api.drone-os.com/drone-core/0.13/)

The foundational crate. It contains functionality shared by all platforms.

**drone-cortexm**
[0.10](https://api.drone-os.com/drone-cortex-m/0.10/)
[0.11](https://api.drone-os.com/drone-cortex-m/0.11/)
[0.12](https://api.drone-os.com/drone-cortexm/0.12/)
[0.13 (latest)](https://api.drone-os.com/drone-cortexm/0.13/)

ARM® Cortex®-M platform crate.

**drone-stm32-map**
[0.10](https://api.drone-os.com/drone-stm32-map/0.10/)
[0.11](https://api.drone-os.com/drone-stm32-map/0.11/)
[0.12](https://api.drone-os.com/drone-stm32-map/0.12/)
[0.13 (latest)](https://api.drone-os.com/drone-stm32-map/0.13/)

STM32 peripheral mappings auto-generated from vendor-provided SVD files.

**drone-nrf-map**
[0.11](https://api.drone-os.com/drone-nrf-map/0.11/)
[0.12](https://api.drone-os.com/drone-nrf-map/0.12/)
[0.13 (latest)](https://api.drone-os.com/drone-nrf-map/0.13/)

Nordic Semi nRFx peripheral mappings auto-generated from vendor-provided SVD
files.

## CLI utility

To get help for Drone's command-line utility or for its subcommands, simply run
the following command in the terminal:

```shell
$ drone help
```
