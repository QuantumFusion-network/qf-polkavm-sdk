#![no_std]
#![no_main]

use parity_scale_codec::{Decode, Encode};

qf_polkavm_sdk::host_functions!();

const STORAGE_KEY_LEN: usize = 256;
type StorageKey = [u8; STORAGE_KEY_LEN];
const READ_BUFFER_LEN: usize = 2048;
type ReadBuffer = [u8; READ_BUFFER_LEN];

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

    let increment = u32::from_be_bytes(buffer[..4].try_into().unwrap());

    call_increment(increment)
}

fn get_input_data(buffer: &mut ReadBuffer) -> Result<(), u64> {
    let pointer: *mut ReadBuffer = buffer;

    match unsafe { get_user_data(pointer as u32) } {
        0 => Ok(()),
        err => Err(err),
    }
}

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
    let storage_key_pointer: *const StorageKey = &storage_key;

    let mut buffer = [0u8; READ_BUFFER_LEN];
    let pointer: *mut ReadBuffer = &mut buffer;

    unsafe {
        match get(storage_key_pointer as u32, pointer as u32) {
            0 => (),
            other => return other,
        }

        let mut tmp: &[u8] = &buffer;
        let mut counter = if buffer == [0u8; READ_BUFFER_LEN] {
            Counter::default()
        } else {
            Counter::decode(&mut tmp).unwrap_or(Counter::default())
        };

        counter.counter = counter.counter.saturating_add(increment);

        for (pos, elem) in Counter::encode(&counter).iter().enumerate() {
            buffer[pos] = *elem
        }

        match set(storage_key_pointer as u32, pointer as u32) {
            0 => (),
            other => return other,
        }
    }

    0
}
