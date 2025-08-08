#[cfg(not(test))]
#[cfg_attr(all(feature = "panic-handler", not(test)), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Safety: The unimp instruction is guaranteed to trap
    unsafe {
        core::arch::asm!("unimp");
        core::hint::unreachable_unchecked();
    }
}
