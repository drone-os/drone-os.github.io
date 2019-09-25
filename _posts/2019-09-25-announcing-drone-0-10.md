---
layout: default
title: "Announcing Drone 0.10"
---

# {{ page.title }}

After more than 2 years of development, we are pleased to announce Drone, an
Embedded Operating System for writing real-time applications in Rust. Drone is a
free software, dual licensed under MIT and Apache 2.0. This is the first public
release, following many iterations of experimentation. Drone's key highlights:

- Supports ARM Cortex-M3/-M4. Also for STM32F1, STM32L4, and STM32L4+ there are
  generated register and interrupt bindings from vendor-provided SVD files.
- Native **async/await** syntax is enabled and is a preferred way for
  concurrency.
- Integrated dynamic memory allocator enables use of `String`, `Vec`, `Box`,
  `Rc`, `Arc`, and more. Still perfectly suitable for low-cost MCUs with 20 KB
  of RAM.
- Rich tooling: generate a new project with `drone new`, configure the memory
  layout in `Drone.toml`, `drone-ld` saves you from manually writing linker
  scripts.
- `print!`, `eprint!` and similar macros are mapped to Cortex-M's ITM channels 0
  and 1 out of the box.
- If using [Black Magic Probe](http://black-magic.org/), `drone bmp itm`
  captures the ITM output and displays it in the console. Also there are `drone
  bmp flash` and `drone bmp gdb` commands.

For more information, please visit [Drone OS official
site](https://www.drone-os.com).
