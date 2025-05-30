//! Example smart contract for printing strings into runtime logs

#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    call_print("hello!")
}

/// Prints string into runtine logs
fn call_print(msg: &str) -> u64 {
    unsafe { print(msg.as_ptr() as u32, msg.len() as u32) }
}
