use crate::consts::{FLASH_WS, PLL_M, PLL_N, PLL_P};
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::reg;

fn init_sys_clk(
    mut rcc_cr: reg::rcc::Cr<Urt>,
    mut rcc_pllcfgr: reg::rcc::Pllcfgr<Urt>,
    mut rcc_cfgr: reg::rcc::Cfgr<Urt>,
    mut flash_acr: reg::flash::Acr<Urt>,
) {
    rcc_cr.modify(|r| r.set_hseon()); // HSE clock enable
    while !rcc_cr.load().hserdy() {} // HSE clock ready flag

    rcc_pllcfgr.store(|r| {
        r
            // HSE oscillator clock selected as PLL and PLLI2S clock entry
            .set_pllsrc()
            // division factor for PLL and PLLI2S input clock
            .write_pllm(PLL_M)
            // PLL multiplication factor for VCO
            .write_plln(PLL_N)
            // PLL division factor for main system clock
            .write_pllp(PLL_P / 2 - 1)
    });
    rcc_cr.modify(|r| r.set_pllon()); // PLL enable
    while !rcc_cr.load().pllrdy() {} // PLL clock ready flag

    flash_acr.store(|r| {
        r
            // the ratio of the CPU clock period to the Flash memory access time
            .write_latency(FLASH_WS)
            // data cache is enabled
            .set_dcen()
            // instruction cache is enabled
            .set_icen()
            // prefetch is enabled
            .set_prften()
    });
    while flash_acr.load().latency() != FLASH_WS {}

    rcc_cfgr.store(|r| {
        r
            // PLL selected as system clock
            .write_sw(0b10)
            // system clock not divided
            .write_hpre(0b0000)
            // APB1 = AHB / 2
            .write_ppre1(0b100)
            // APB2 = AHB / 1
            .write_ppre2(0b000)
    });
    while rcc_cfgr.load().sws() != 0b10 {} // PLL used as the system clock
}
