use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ExprPath};

#[proc_macro_attribute]
pub fn Stmt(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let node_type = parse_macro_input!(attr as ExprPath);
    let name = &input.ident;

    let expanded = quote! {
        #[derive(Debug, Clone)]
        #input

        impl Stmt for #name {
            fn kind(&self) -> NodeType {
                #node_type
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn clone_box(&self) -> Box<dyn Stmt> {
                Box::new(self.clone())
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
        #[derive(Debug, Clone)]
        #input

        impl Expr for #name {
            fn clone_box_expr(&self) -> Box<dyn Expr> {
                Box::new(self.clone())
            }
        }

        impl Stmt for #name {
            fn kind(&self) -> NodeType {
                #node_type
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn clone_box(&self) -> Box<dyn Stmt> {
                Box::new(self.clone())
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_attribute]
pub fn RuntimeValue(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let node_type = parse_macro_input!(attr as ExprPath);
    let name = &input.ident;

    let expanded = quote! {
        #[derive(Debug)]
        #input

        impl RuntimeValue for #name {
            fn Type(&self) -> RuntimeValueType {
                #node_type
            }
        }
    };

    TokenStream::from(expanded)
}
