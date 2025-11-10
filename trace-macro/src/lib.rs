use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::{ItemFn, parse_macro_input};

const CRATE_NAME: &str = "trace";

/// Helper macro to allow a user to define an extern trace_write function
/// with a closure
#[proc_macro_attribute]
pub fn trace_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;

    let crate_path = match crate_name(CRATE_NAME) {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote!(#ident)
        }
        Err(_) => panic!("Could not find the `{}` crate.", CRATE_NAME),
    };

    // Validate the signature
    let args: Vec<_> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => Some(pat_type),
            syn::FnArg::Receiver(_) => None, // ignore `self`
        })
        .collect();

    if args.len() != 3 {
        return syn::Error::new_spanned(
            &input.sig,
            "#[trace_handler] functions must have exactly two arguments: (level: u8, msg: *const u8, msg_len: usize)",
        )
        .to_compile_error()
        .into();
    }

    let expanded = quote! {
        #input

        // Export an extern entry point for the trace function
        #[unsafe(no_mangle)]
        pub unsafe extern "Rust" fn _on_trace(level: #crate_path::Level, msg: *const u8, msg_len: usize) {
            #name(level, msg, msg_len);
        }
    };

    expanded.into()
}
