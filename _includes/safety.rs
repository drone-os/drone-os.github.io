/// The entry point.
///
/// # Safety
///
/// This function should only be called by hardware.
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    // Fill the memory region allocated for initially zeroed mutable static
    // variables with zeros. This is safe because none of these variables have
    // been in use before this line.
    unsafe { mem::bss_init() };
    // Fill the memory region for other mutable static variables with initial
    // values from flash memory. This is safe because none of these variables
    // have been in use before this line.
    unsafe { mem::data_init() };
    // Initialize the Floating Point Unit. This is safe because the unit has not
    // been in use before this line.
    unsafe { processor::fpu_init(true) };
    // Run the root task.
    tasks::root(
        // Instantiate a zero-sized collection of tokens for memory-mapped
        // registers. Safe only if this is the only instance.
        unsafe { Regs::take() },
        // Instantiate a zero-sized token needed to initialize the threading
        // system later. Safe only if this is the only instance.
        unsafe { ThrsInit::take() },
    );
    // If the root task returned, always sleep between interrupts.
    loop {
        processor::wait_for_int();
    }
}
