---
layout: default
title: "Announcing Drone 0.14"
---

# {{ page.title }}

The main focus of this release was adding experimental support for the new
[RISC-V](https://riscv.org/) hardware architecture. We had two dev boards to
experiment with: [SiFive HiFive1 Rev
B](https://www.sifive.com/boards/hifive1-rev-b) and [Sipeed Longan
Nano](https://www.seeedstudio.com/Sipeed-Longan-Nano-RISC-V-GD32VF103CBT6-Development-Board-p-4205.html). Both
were running on RV32IMAC variant RISC-V cores. The "A" letter means that the
core implements the Atomic Instructions Standard Extension. And while the
HiFive1's core implements this extension on paper, it has a critical
limitation - `LR`/`SC` instructions, which are used to implement the atomic CAS
(compare-and-swap) operation, are not supported in the RAM region. Making it not
compatible with Drone requirements for hardware atomics. However the Longan
Nano's core implements the Atomic Extension correctly and without limitations,
hence it becomes the first RISC-V board that runs Drone OS, and here is the
blinking demo application:
[longan-blink](https://github.com/drone-os/longan-blink).

RISC-V specification describes a very basic interrupt controller named CLINT. It
lacks crucial features that other controllers like Cortex-M's NVIC have, namely
prioritization and pre-emption. We decided to implement the missing pieces in
software. Hence we introduce a new `drone_core::thr::soft!` macro and
`drone_core::thr::soft` module. The basic `drone_core::thr!` becomes
`drone_core::thr::pool!`, Cortex-M's `drone_cortexm::thr!` becomes
`drone_cortexm::thr::nvic!`, and RISC-V's variant is `drone_riscv::clint!`.

Other important changes include:

* Multiple heaps support
* Multiple thread pools support
* Heap tracing can be enabled via editing the `heap!` macro instead of relying
  on cargo features
* Expose generated linker scripts at `target/<target>/layout.ld.<stage>`
* A new synchronization primitive: lock-free singly-linked list:
  `drone_core::sync::LinkedList`
* `drone_core::sync::Mutex` has been rewritten to be futures-aware
* A long-awaited context-switching optimization: removed one layer of
  indirection in `drone_core::fib::Chain`
* `drone_core::thr::local` function for accessing TLS (thread local storage) is
  moved to `drone_core::thr::Thread` trait and now can return `None` if called
  outside of the corresponding thread pool

There are other changes in the Drone 0.14 release: check out what changed in
[drone](https://github.com/drone-os/drone/blob/master/CHANGELOG.md#v0140-2021-04-09),
[drone-core](https://github.com/drone-os/drone-core/blob/master/CHANGELOG.md#v0140-2021-04-09),
[drone-cortexm](https://github.com/drone-os/drone-core/blob/master/CHANGELOG.md#v0140-2021-04-09),
and
[drone-stm32-map](https://github.com/drone-os/drone-stm32-map/blob/master/CHANGELOG.md#v0140-2021-04-09).
