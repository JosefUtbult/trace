use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Helper macro to allow a user to define an extern trace_write function
/// with a closure
#[proc_macro_attribute]
pub fn trace_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;

    // Validate the signature
    if input.sig.inputs.len() != 1 {
        return syn::Error::new_spanned(
            &input.sig,
            "#[trace_handler] functions must have exactly one argument: (&str)",
        )
        .to_compile_error()
        .into();
    }

    let expanded = quote! {
        #input

        // Export an extern entry point for the trace function
        #[unsafe(no_mangle)]
        pub unsafe extern "Rust" fn _on_trace(msg: &str) {
            #name(msg);
        }
    };

    expanded.into()
}
