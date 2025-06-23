use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ExprPath};

#[proc_macro_attribute]
pub fn Stmt(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let node_type = parse_macro_input!(attr as ExprPath);
    let name = &input.ident;

    let expanded = quote! {
        #[derive(Debug)]
        #input

        impl Stmt for #name {
            fn kind(&self) -> NodeType {
                #node_type
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn Expr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let node_type = parse_macro_input!(attr as ExprPath);
    let name = &input.ident;

    let expanded = quote! {
        #[derive(Debug)]
        #input

        impl Expr for #name {}

        impl Stmt for #name {
            fn kind(&self) -> NodeType {
                #node_type
            }
        }
    };

    TokenStream::from(expanded)
}

