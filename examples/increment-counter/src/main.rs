//! Example of working with storage.
//! Includes reading and writing to storage cell,
//! encoding and decoding a simple structure to SCALE format,
//! loading u32 number from user input.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use codec::{Decode, Encode};
use pallet_revive_uapi::{input, unwrap_output, HostFn, HostFnImpl as api, StorageFlags};

use qf_polkavm_sdk::prelude::*;

const KEY: [u8; 32] = [1u8; 32];

#[export]
pub fn deploy() {
    // Initialize storage counter with 0.
    api::set_storage(StorageFlags::empty(), &KEY, &0u32.encode());
}

#[export]
pub fn call() {
    // Accept increment value from user input. Should be 4 bytes (e.g., `0x12345678`) or `ContractTrapped` error occurs.
    input!(increment: u32, );

    // Read the current value from storage.
    unwrap_output!(
        raw_data,
        [0u8; 4],
        api::get_storage,
        StorageFlags::empty(),
        &KEY
    );
    let old = u32::decode(&mut &raw_data[..]).unwrap();

    // Increment the value and write it back to storage.
    let (new, _) = old.overflowing_add(increment);
    api::set_storage(StorageFlags::empty(), &KEY, &new.encode());

    // Emit the update event with the old and new values.
    api::deposit_event(
        &[],
        format!("Counter incremented by {increment} from {old} to {new}.").as_bytes(),
    );
}
