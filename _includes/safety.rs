#[no_mangle]
#[naked]
pub unsafe extern "C" fn reset() -> ! {
    mem::bss_init();
    mem::data_init();
    processor::fpu_init(true);
    tasks::root(Regs::take(), ThrsInit::take());
    loop {
        processor::wait_for_int();
    }
}
