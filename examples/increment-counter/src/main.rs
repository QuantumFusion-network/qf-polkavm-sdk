//! Minimal example smart contract. It uses allocator and panic handler from the qf-polkavm-sdk and
//! works with storage via pallet-revive-uapi for reading and writing SCALE-encoded data.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

// Data (de)serialization used in Polkadot ecosystem. You are free to bring your own as well, but
// don't expect compatibility with the ecosystem tools (e.g., client libraries, UI) if you do so.
use codec::{Decode, Encode};

// Adds host VM interaction functions and related types into scope.
use pallet_revive_uapi::{input, unwrap_output, HostFn, HostFnImpl as api, StorageFlags};

// Adds global allocator and panic handler, also brings `export` macro into scope.
use qf_polkavm_sdk::prelude::*;

// Host VM provides key-value storage for contracts, this storage key is for storing the counter
// value.
const KEY: [u8; 32] = [1u8; 32];

// This function called once during the smart contract deployment transaction execution.
#[export]
pub fn deploy() {
    // Initialize storage counter with 0.
    api::set_storage(StorageFlags::empty(), &KEY, &0u32.encode());
}

// This function called on each smart contract invocation.
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
