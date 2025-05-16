#![no_std]
#![no_main]

use parity_scale_codec::{Decode, Encode};

qf_polkavm_sdk::host_functions!();

#[derive(Encode, Decode, Default)]
struct Counter {
    counter: u32,
}

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    let mut buffer = [0u8; 64];
    if let Err(err) = get_input_data(&mut buffer) {
        return err;
    }

    let increment = u32::from_be_bytes(buffer[..4].try_into().unwrap());

    call_increment(increment)
}

fn get_input_data(buffer: &mut [u8; 64]) -> Result<(), u64> {
    let pointer: *mut [u8; 64] = buffer;

    match unsafe { get_user_data(pointer as u32) } {
        0 => Ok(()),
        err => Err(err),
    }
}

fn call_increment(increment: u32) -> u64 {
    // "                                                             foo"
    // "0x20202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020666F6F"
    let storage_key: [u8; 64] = [
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 102, 111, 111,
    ];
    let storage_key_pointer: *const [u8; 64] = &storage_key;

    let mut buffer = [0u8; 64];
    let pointer: *mut [u8; 64] = &mut buffer;

    unsafe {
        match get(storage_key_pointer as u32, pointer as u32) {
            0 => (),
            other => return other,
        }

        let mut tmp: &[u8] = &buffer;
        let mut counter = if buffer == [0u8; 64] {
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
