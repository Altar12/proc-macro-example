extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(PrintMe)]
pub fn print_me_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_print_me(&ast)
}

fn impl_print_me(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let code = quote! {
        impl PrintMe for #name {
            fn print_me() {
                println!("Hello, my name is {}", stringify!(#name));
            }
        }
    };
    code.into()
}
