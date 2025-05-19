#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    call_transfer()
}

fn call_transfer() -> u64 {
    let result = unsafe { transfer(2, 0) };
    if result == 0 {
        0
    } else {
        panic!("Failed transfer");
    }
}
