#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = match ::syn::parse::<DeriveInput>(input) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    let command_name = input.ident;
    let builder_name = {
        let res = ::alloc::fmt::format(format_args!("{0}Builder", command_name));
        res
    };
    let builder_ident = syn::Ident::new(&builder_name, command_name.span());
    TokenStream::new()
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
