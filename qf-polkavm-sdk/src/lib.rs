//! Native Rust smart contracts SDK for QF Network. Smart contract functionality in QF Network is
//! implemented using `pallet-revive` from Polkadot SDK / Substrate and this SDK should work for
//! creating smart contracts for other blockchains that integrate `pallet-revive` as well.

#![no_std]
#![no_main]

#[cfg(feature = "global-allocator")]
pub mod bump_allocator;

#[cfg(feature = "panic-handler")]
pub mod panic_handler;

pub mod prelude;
