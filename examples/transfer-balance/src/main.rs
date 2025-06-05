//! Example of transferring tokens inside smart contract.
//! This example transfers tokens to address from `to` argument.
//! We get value from `value` argument.

#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    call_transfer()
}

fn call_transfer() -> u64 {
    // `to` have index 2 in account vector
    // balance have index 0 in balances vector
    // transfer(address_idx: u32, balance_idx: u32)
    let result = unsafe { transfer(2, 0) };
    if result == 0 {
        0
    } else {
        panic!("Failed transfer");
    }
}
