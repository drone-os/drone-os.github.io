---
layout: default
title: "Announcing Drone 0.11"
---

# {{ page.title }}

## New target devices

- Nordic's nRF52 micro-controller family
- ST's STM32F4 micro-controller family

## New debug probes

Added support for [OpenOCD](http://openocd.org/) debugging interface. `drone
bmp` command was renamed to a more general `drone probe`. `[bmp]` section was
removed from `Drone.toml` in favor of a new `[probe]` section.

OpenOCD enables use of ST-Link probes (and many others, see the relevant
[docs](http://openocd.org/supported-jtag-interfaces/)) with internal or external
(using generic USB-to-UART converter) ITM capture.

## New crates

- `drone-nrf-map` for Nordic's nRFx chips
- `drone-svd` was extracted from `drone-stm32-map`, and is used as a base for
  both `drone-stm32-map` and `drone-nrf-map`

## New features

`Drone.toml` supports a new `linker.include` configuration value. Each file from
this array is included to a final auto-generated linker script. This is useful
when you link an external object file, which has custom linker sections.

## Changes

`drone-cortex-m`, `drone-stm32-map`, and `drone-nrf-map` are using custom
condition compilation flags instead of cargo features for choosing the processor
model. This allowed use of cargo features for excluding unneeded peripheral
sub-crates from compilation. Generated `Justfile` was changed to use a new
`drone env` command for setting correct environment variables.

Register's `to_ptr`/`to_mut_ptr` methods was changed to `as_ptr`/`as_mut_ptr` as
they are more precisely reflects their signatures.
