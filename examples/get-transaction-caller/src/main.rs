//! Example of getting current transaction caller address.
//! Ex if transaction was executed by ALICE, it will print it's address.
//! Loads address and prints it to the runtime logs in SS58 format.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, vec::Vec};

use codec::Decode;
use sp_core::crypto::{AccountId32, Ss58Codec};

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    let scale_bytes = get_caller_address();

    // Decode SCALE into address
    let account_id = AccountId32::decode(&mut &scale_bytes[..]).unwrap();
    // Format into SS58
    let ss58_address = account_id.to_ss58check();
    call_print(&format!("caller address: {ss58_address}"))
}

/// Get address of caller.
/// Loads bytes from caller() index in accounts vector.
fn get_caller_address() -> Vec<u8> {
    // Get caller index
    let id = unsafe { caller() } as u32;

    // Get length of SCALE encoded address in bytes
    let len = unsafe { get_address_len(id) } as usize;

    // Allocate buffer to read
    let mut address_buffer: Vec<u8> = Vec::with_capacity(len);
    address_buffer.resize(len, 0);

    // Call host function to load encoded address
    if unsafe { get_address(id, address_buffer.as_mut_ptr() as u32) } != 0 {
        panic!();
    }

    address_buffer
}

/// Prints string into runtine logs
fn call_print(msg: &str) -> u64 {
    unsafe { print(msg.as_ptr() as u32, msg.len() as u32) }
}
