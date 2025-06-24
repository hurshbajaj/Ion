#![feature(trait_upcasting)]

use std::fs;
mod lexer;
mod parser;
mod ast;

mod runtime;
use runtime::*;

use crate::scopes::Scope;
use crate::values::{BooleanVal, NilVal, NumericVal};

fn main() {
    let source = fs::read_to_string("code.io")
        .expect("Failed to read file 'code.io'");

    unsafe{
        let mut env = Scope::new(scopes::Parent::Nil);
        env.var_decl("test_var".to_string(), Box::new(NumericVal::<f64>{value: 5.0}));

        let output = parser::prodAST(source);
        println!("Abstract Syntax Tree: {:?}", output);
        let evaluated = interpreter::evaluate(Box::new(output), &env);
        println!("Runtime Value Debug: {:?}", evaluated);

    }

}
