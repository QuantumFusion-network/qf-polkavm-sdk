#![no_std]
#![no_main]

qf_polkavm_sdk::host_functions!();

#[polkavm_derive::polkavm_export]
extern "C" fn main(op: u32) -> u64 {
    match op {
        0 => call_transfer(),
        1 => call_balance(),
        3 => call_block_number(),
        2 => call_balance_of(),
        4 => call_infinity_loop(),
        5 => call_inc(),
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

fn call_inc() -> u64 {
    let mut buffer = [0u8; 8];
    let pointer: *mut [u8; 8] = &mut buffer;

    unsafe {
        if get(pointer as u32, 0u32) != 0 {
            return 1
        };

        buffer[7] += 1;

        if set(pointer as u32, 0u32) != 0 {
            return 1
        }
    }

    0
}
