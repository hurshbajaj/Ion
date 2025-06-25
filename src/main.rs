#![feature(trait_upcasting)]

use std::cell::RefCell;
use std::{fs, panic};
mod lexer;
mod parser;
mod ast;

mod runtime;
use runtime::*;

use crate::scopes::Scope;
use crate::values::{BooleanVal, NilVal, NumericVal};

fn main() {
    let source = fs::read_to_string("main.io")
        .expect("Failed to read file 'main.io'");

    unsafe{
        let mut env = RefCell::new(Scope::new(scopes::Parent::Nil));

        let output = parser::prodAST(source);
        println!("Abstract Syntax Tree: {:?}", output);
        let evaluated = interpreter::evaluate(Box::new(output), &env);
        println!("Runtime Value Debug: {:?}", evaluated);

    }

}
