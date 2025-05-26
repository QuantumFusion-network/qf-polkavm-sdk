#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    let contract = get_conatract_account();
    let msg = format!("contract: {contract}");
    call_print(&msg)
}

fn get_conatract_account() -> u64 {
    unsafe { account_id() }
}

fn call_print(msg: &str) -> u64 {
    let msg_pointer: *const u8 = msg.as_ptr();
    let len = msg.len();

    unsafe { print(msg_pointer as u32, len as u32) }
}
