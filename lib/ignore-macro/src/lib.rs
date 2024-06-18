use assert_cmd::Command;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input};

#[proc_macro_attribute]
pub fn test_only_exists_binary(attr: TokenStream, input: TokenStream) -> TokenStream {
    let bin_str = attr.to_string();
    let bin_name = bin_str.trim_matches('"');
    let mut item = parse_macro_input!(input as syn::ItemFn);

    item.attrs.push(syn::parse_quote! {#[test]});

    match Command::cargo_bin(bin_name) {
        Ok(_) => quote! {#item}.into(),
        Err(_) => {
            item.attrs.push(syn::parse_quote! {
                #[ignore = "Not Solved Yet"]
            });

            quote! {#item}.into()
        }
    }
}
