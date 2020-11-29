---
layout: default
title: "Announcing Drone 0.13"
---

# {{ page.title }}

This is an intermediate release on the way of adding RISC-V architecture
support.

To compile on current Rust nightly, a breaking change was needed. Previously
`drone_core::inventory` module was based on `const_generics` unstable
feature. But lately it was noticeably limited. We decided to use `typenum` crate
instead, which slightly changed public signatures of `inventory` module.

Many macros, including `thr!`, `heap!`, `reg!`, has changed their syntax. The
goal of new syntax is to make Drone macros self-documenting, intuitive and
uniform.

Items deprecated in version 0.12 was removed. This includes some stream methods
on thread tokens and `drone_cortexm::drv::fpu` module.

Interrupt bindings, which were used for type-checking interrupt numbers, were
removed. There were two reasons: many vendor SVD files tend to contain garbage
in interrupt descriptions, and this feature was useless in generic contexts.

Also this release adds a new experimental target - Texas Instruments CC2538, and
a new experimental crate -
[drone-tisl-map](https://github.com/drone-os/drone-tisl-map), which is the first
Drone crate driven solely by the community!
