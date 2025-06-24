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
    let node_type = attr.clone();       
    let item_clone = item.clone();      

    let stmt_expanded = Stmt(node_type, item_clone);
    let stmt_expanded2: proc_macro2::TokenStream = stmt_expanded.into();

    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let expr_expanded = quote! {
        impl Expr for #name {
            fn clone_box_expr(&self) -> Box<dyn Expr> {
                Box::new(self.clone())
            }
        }
    };

    let combined = quote! {
        #stmt_expanded2
        #expr_expanded
    };

    TokenStream::from(combined)
}


#[proc_macro_attribute]
pub fn RuntimeValue(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let node_type = parse_macro_input!(attr as ExprPath);
    let name = &input.ident;

    let expanded = quote! {
        #[derive(Debug, Clone)]
        #input

        impl RuntimeValue for #name {
            fn Type(&self) -> RuntimeValueType {
                #node_type
            }
            fn clone_box(&self) -> Box<dyn RuntimeValue> {
                Box::new(self.clone())
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}
