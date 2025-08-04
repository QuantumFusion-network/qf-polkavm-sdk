#![no_std]
#![no_main]

use codec::{Decode, Encode};

qf_polkavm_sdk::host_functions!();

#[derive(Encode, Decode)]
enum Command {
    Transfer,     // 0x00
    Balance,      // 0x01
    BalanceOf,    // 0x02
    BlockNumber,  // 0x03
    InfinityLoop, // 0x04
    Inc,          // 0x05
    Delete,       // 0x06
}

#[derive(Encode, Decode, Default)]
struct Counter {
    counter: u32,
}

#[polkavm_derive::polkavm_export]
extern "C" fn main() -> u64 {
    let mut buffer = [0u8; 2048];
    let pointer: *mut [u8; 2048] = &mut buffer;

    let command: Command;

    unsafe {
        match get_user_data(pointer as u32) {
            0 => (),
            other => return other,
        }

        let mut tmp: &[u8] = &buffer;
        if let Ok(new_command) = Command::decode(&mut tmp) {
            command = new_command;
        } else {
            return 1;
        }
    }

    match command {
        Command::Transfer => call_transfer(),
        Command::Balance => call_balance(),
        Command::BalanceOf => call_balance_of(),
        Command::BlockNumber => call_block_number(),
        Command::InfinityLoop => call_infinity_loop(),
        Command::Inc => call_inc(),
        Command::Delete => call_delete(),
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

fn call_inc() -> u64 {
    // string representation of the key used to store the counter value
    // "                                                                                                                                                                                                                                                             foo"

    // hex representation of the key used to store the counter value
    // "0x20202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020666F6F"

    // binary representation of the key used to store the counter value
    let storage_key: [u8; 256] = [
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
    let storage_key_pointer: *const [u8; 256] = &storage_key;

    let mut buffer = [0u8; 2048];
    let pointer: *mut [u8; 2048] = &mut buffer;

    unsafe {
        match get(storage_key_pointer as u32, pointer as u32) {
            0 => (),
            other => return other,
        }

        let mut tmp: &[u8] = &buffer;
        let mut counter = if buffer == [0u8; 2048] {
            Counter::default()
        } else {
            Counter::decode(&mut tmp).unwrap_or(Counter::default())
        };

        counter.counter += 1;

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

fn call_delete() -> u64 {
    // string representation of the key used to store the counter value
    // "                                                                                                                                                                                                                                                             foo"

    // hex representation of the key used to store the counter value
    // "0x20202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020202020666F6F"

    // binary representation of the key used to store the counter value
    let storage_key: [u8; 256] = [
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
    let storage_key_pointer: *const [u8; 256] = &storage_key;

    unsafe {
        let delete_result = delete(storage_key_pointer as u32);
        if delete_result != 0 {
            return delete_result;
        };
    }

    0
}
