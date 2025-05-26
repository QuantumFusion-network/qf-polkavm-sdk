#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    call_print("hello!")
}

fn call_print(msg: &str) -> u64 {
    let msg_pointer: *const u8 = msg.as_ptr();
    let len = msg.len();

    unsafe { print(msg_pointer as u32, len as u32) }
}
