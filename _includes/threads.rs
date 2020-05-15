use crate::thr;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::reg;

async fn enable_hse_clock(
    rcc_cir: reg::rcc::Cir<Srt>,
    rcc_cr: reg::rcc::Cr<Srt>,
    thr_rcc: thr::Rcc,
) {
    // We need to move ownership of `hserdyc` and `hserdyf` into the following
    // fiber.
    let reg::rcc::Cir { hserdyc, hserdyf, .. } = rcc_cir;
    // Attach a listener that will notify us when RCC_CIR_HSERDYF is asserted.
    let hserdy = thr_rcc.add_future(fib::new_fn(move || {
        if hserdyf.read_bit() {
            hserdyc.set_bit();
            fib::Complete(())
        } else {
            fib::Yielded(())
        }
    }));
    // Enable the HSE clock.
    rcc_cr.modify(|r| r.set_hseon());
    // Sleep until RCC_CIR_HSERDYF is asserted.
    hserdy.await;
}
