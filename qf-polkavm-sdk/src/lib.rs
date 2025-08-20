#![no_std]
#![no_main]

#[cfg(feature = "global-allocator")]
pub mod bump_allocator;

#[cfg(feature = "panic-handler")]
pub mod panic_handler;

pub mod prelude;
