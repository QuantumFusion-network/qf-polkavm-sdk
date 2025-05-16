#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    call_block_number()
}

fn call_block_number() -> u64 {
    unsafe { block_number() }
}
