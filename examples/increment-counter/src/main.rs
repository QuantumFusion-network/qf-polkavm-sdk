//! Example of working with storage.
//! Includes reading and writing to storage cell,
//! encoding and decoding a simple structure to SCALE format,
//! loading u32 number from user input.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

use parity_scale_codec::{Decode, Encode};

qf_polkavm_sdk::host_functions!();

const STORAGE_KEY_LEN: usize = 256;
type StorageKey = [u8; STORAGE_KEY_LEN];
const READ_BUFFER_LEN: usize = 2048;
type ReadBuffer = [u8; READ_BUFFER_LEN];

/// Counter stored in storage and incremented by smart contract
/// by number provided from user
#[derive(Encode, Decode, Default)]
struct Counter {
    counter: u32,
}

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    let mut buffer = [0u8; READ_BUFFER_LEN];

    if let Err(err) = get_input_data(&mut buffer) {
        return err;
    }

    let increment = match u32::decode(&mut &buffer[..]) {
        Ok(value) => value,
        Err(err) => {
            call_print(&format!("decoding failed: {err:?}"));
            return 1;
        }
    };

    call_increment(increment)
}

/// Get data pssed into smart contract call and save into buffer
fn get_input_data(buffer: &mut ReadBuffer) -> Result<(), u64> {
    let pointer: *mut ReadBuffer = buffer;

    match unsafe { get_user_data(pointer as u32) } {
        0 => Ok(()),
        err => Err(err),
    }
}

/// Increment counter by specified `increment` number
fn call_increment(increment: u32) -> u64 {
    // string representation of the key used to store the counter value
    // "                                                                                                                                                                                                                                                             foo"

    // hex representation of the key used to store the counter value
    // "0x20202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020666F6F"

    // binary representation of the key used to store the counter value
    let storage_key: StorageKey = [
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        102, 111, 111,
    ];
    let mut buffer = [0u8; READ_BUFFER_LEN];

    unsafe {
        // Read from storage
        match get(storage_key.as_ptr() as u32, buffer.as_mut_ptr() as u32) {
            0 => (),
            err => return err,
        }

        // Decode counter structure
        let mut counter = if buffer == [0u8; READ_BUFFER_LEN] {
            // Default value if zero
            Counter::default()
        } else {
            Counter::decode(&mut &buffer[..]).unwrap_or(Counter::default())
        };

        // Increment by value
        let new_value = counter.counter.saturating_add(increment);
        call_print(&format!(
            "incrementing counter: {} + {} -> {}",
            counter.counter, increment, new_value
        ));
        counter.counter = new_value;

        // Encode counter structure
        buffer.fill(0);
        for (pos, elem) in Counter::encode(&counter).iter().enumerate() {
            buffer[pos] = *elem
        }

        // Write to storage
        match set(storage_key.as_ptr() as u32, buffer.as_mut_ptr() as u32) {
            0 => (),
            err => return err,
        }
    }

    0
}

/// Prints string into runtine logs
fn call_print(msg: &str) -> u64 {
    unsafe { print(msg.as_ptr() as u32, msg.len() as u32) }
}
