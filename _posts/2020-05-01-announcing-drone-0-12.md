---
layout: default
title: "Announcing Drone 0.12"
---

# {{ page.title }}

One of the biggest goals for this release of Drone was adding support for
**Nordic's nRF9160** target. nRF9160 is built on top of new **Cortex-M33** core,
which in turn is built on top of new **ARMv8-M** architecture.

`drone-cortexm` has gained support for Cortex-M33. The new core no longer
implements bit-banding, therefore this functionality was moved behind the new
`bit-band` feature flag. For MCUs that implement Cortex-M33's **Security
Extension**, a new `security-extension` feature was added.

Cortex-M33 with Security Extension heavily uses registers aliasing. In order to
keep Drone's memory-mapped register API sound, it was extended to support
**multiple variants of one register**. This allows freely switching between
register variants, ensuring that only one variant is active at a time.

nRF9160 DK (Development Kit) has **Segger J-Link** OB debugger
on-board. Therefore we decided to add an initial support for J-Link debug
probes. Currently it's supported only for nRF9160, but other targets are
planned.

nRF9160 doesn't implement SWO, instead it has a parallel trace port. However,
there is a few downsides for using the parallel port: it requires dedicated and
expensive trace probes, and in case of nRF9160 DK, it requires manual soldering
of the connector. On the other hand, nRF9160 DK has a built-in USB-serial
adapter, which is convenient to capture nRF9160's UART output. Therefore, in
addition to the existing SWO log type, we decided to add a new log type named
**DSO (Drone Serial Output)**. DSO is implemented on top of a generic UART
peripheral, and uses a dedicated DSO protocol to mimic SWO behavior, such as
splitting the output into multiple streams. Currently DSO is implemented only
for nRF9160 with help of `drone-nrf91-dso` crate.

To incorporate multiple log types, a **new `drone_core::log` facade** was
introduced. Much like the semi-official [log](https://crates.io/crates/log)
crate, `drone_core::log` abstracts over the actual logging implementation. This
allows to use standard logging capabilities (e.g. `print!`, `dbg!` macros), in
hardware independent crates, i.e. such that depends only on `drone_core`. The
actual logging implementation is set in the final application by using
`swo::set_log!()` macro or similar.

`libcore-drone` wrapper crate was dropped in favor of the **native
`async`/`await` in `no_std`** support, which uses new generator resume arguments
feature under the hood, instead of thread-local storage.

Finally, Drone's **debugging story** was improved. For newly generated projects,
`release` profile includes debug symbols. These symbols are not written into the
device flash memory, but helps GDB to map your source code to the running
target. Rust's standard `core`/`alloc` source code mapping was also improved. A
new `gdb-mi` task was added to generated `Justfile`s for using as a drop-in
replacement of `gdb -i=mi` command in a debugger GUI or an IDE.

There are other changes in the Drone 0.12 release: check out what changed in
[drone](https://github.com/drone-os/drone/blob/master/CHANGELOG.md#v0120-2020-05-01),
[drone-core](https://github.com/drone-os/drone-core/blob/master/CHANGELOG.md#v0120-2020-05-01),
[drone-cortexm](https://github.com/drone-os/drone-cortexm/blob/master/CHANGELOG.md#v0120-2020-05-01),
[drone-stm32-map](https://github.com/drone-os/drone-stm32-map/blob/master/CHANGELOG.md#v0120-2020-05-01),
and
[drone-nrf-map](https://github.com/drone-os/drone-nrf-map/blob/master/CHANGELOG.md#v0120-2020-05-01).
