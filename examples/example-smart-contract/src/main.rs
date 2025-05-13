#![no_std]
#![no_main]

use parity_scale_codec::{Encode, Decode};

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main(op: u32) -> u64 {
    match op {
        0 => call_transfer(),
        1 => call_balance(),
        2 => call_balance_of(),
        3 => call_block_number(),
        4 => call_infinity_loop(),
        5 => call_inc(),
        6 => call_delete(),
        _ => unimplemented!(),
    }
}

fn call_balance() -> u64 {
    unsafe { balance() }
}

fn call_balance_of() -> u64 {
    unsafe { balance_of() }
}

fn call_block_number() -> u64 {
    unsafe { block_number() }
}

fn call_infinity_loop() -> u64 {
    loop {}
}

fn call_transfer() -> u64 {
    unsafe { transfer(2, 0) }
}

#[derive(Encode, Decode, Default)]
struct Counter {
    counter: u32,
}

fn call_inc() -> u64 {
    // "                                                             foo"
    // "0x20202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020666F6F"
    let storage_key: [u8; 64] = [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 102, 111, 111];
    let storage_key_pointer: *const [u8; 64] = &storage_key;

    let mut buffer = [0u8; 64];
    let pointer: *mut [u8; 64] = &mut buffer;

    unsafe {
        let get_result = get(storage_key_pointer as u32, pointer as u32);
        if get_result != 0 {
            return get_result
        };

        let mut tmp: &[u8] = &buffer;
        let mut counter = if buffer == [0u8; 64] {
            Counter::default()
        } else {
            Counter::decode(&mut tmp).unwrap_or(Counter::default())
        };

        counter.counter += 1;

        for (pos, elem) in Counter::encode(&counter).iter().enumerate() {
            buffer[pos] = *elem
        }

        let set_result = set(storage_key_pointer as u32, pointer as u32);
        if set_result != 0 {
            return set_result
        }
    }

    0
}

fn call_delete() -> u64 {
    // "                                                             foo"
    // "0x20202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020666F6F"
    let storage_key: [u8; 64] = [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 102, 111, 111];
    let storage_key_pointer: *const [u8; 64] = &storage_key;

    unsafe {
        let delete_result = delete(storage_key_pointer as u32);
        if delete_result != 0 {
            return delete_result
        };
    }

    0
}
