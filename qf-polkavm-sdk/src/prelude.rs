/// Exports a function for PolkaVM by adding `#[no_mangle]`, `#[polkavm_derive::polkavm_export]`,
/// and `extern "C"` calling convention.
pub use qf_polkavm_sdk_procedural::export;
