#![no_std]
#![no_main]

#[macro_export]
macro_rules! host_functions {
    () => {
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            unsafe {
                core::arch::asm!("unimp", options(noreturn));
            }
        }

        #[polkavm_derive::polkavm_import]
        extern "C" {
            fn balance() -> u64;
            fn balance_of() -> u64;
            fn print() -> u64;
            fn transfer(address_idx: u32, balance_idx: u32) -> u64;
            fn block_number() -> u64;
            fn account_id() -> u64;
            fn caller() -> u64;
        }
    }
}