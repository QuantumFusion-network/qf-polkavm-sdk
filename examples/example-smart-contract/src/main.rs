#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main(op: u32) -> u64 {
    match op {
        0 => call_transfer(),
        1 => call_balance(),
        2 => call_balance_of(),
        3 => call_block_number(),
        4 => call_infinity_loop(),
        _ => unimplemented!(),
    }
}

fn call_balance() -> u64 {
    unsafe { balance() }
}

fn call_balance_of() -> u64 {
    unsafe { balance_of() }
}

fn call_block_number() -> u64 {
    unsafe { block_number() }
}

fn call_infinity_loop() -> u64 {
    loop {}
}

fn call_transfer() -> u64 {
    unsafe { transfer(2, 0) }
}
