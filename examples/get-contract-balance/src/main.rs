//! Example of getting current smart contract balance.
//! Returns number of tokens into result code.

#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    call_balance()
}

fn call_balance() -> u64 {
    unsafe { balance() }
}
