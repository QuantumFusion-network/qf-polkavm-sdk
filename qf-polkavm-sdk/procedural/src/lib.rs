use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

///
/// ---
///
/// Documentation for this macro can be found at `qf_polkavm_sdk::prelude::export`.
#[proc_macro_attribute]
pub fn export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_attrs = &input_fn.attrs;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        #(#fn_attrs)*
        #[no_mangle]
        #[polkavm_derive::polkavm_export]
        #fn_vis extern "C" #fn_sig #fn_block
    };

    TokenStream::from(expanded)
}
