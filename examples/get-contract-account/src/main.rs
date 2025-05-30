//! Example for getting address of current smart contract.
//! Prints address into runtime logs in SS58 format.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, vec::Vec};

use codec::Decode;
use sp_core::crypto::{AccountId32, Ss58Codec};

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    let scale_bytes = get_contract_address();

    let account_id = AccountId32::decode(&mut &scale_bytes[..]).unwrap();
    let ss58_address = account_id.to_ss58check();
    call_print(&format!("contract address: {ss58_address}"))
}

/// Get address of current contract.
/// Loads bytes from account_id() index in accounts vector.
fn get_contract_address() -> Vec<u8> {
    // Get contract index
    let id = unsafe { account_id() } as u32;

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
