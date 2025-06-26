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

    unsafe {
        let mut env = RefCell::new(Scope::new(scopes::Parent::Nil));

        // Stage 1: Show Original Source
        if print_ {
            println!("\n-------------------------- Original -------------------------------\n");
            println!("{}\n", source);
        }

        // Stage 2: Parse to AST
        let output = parser::prodAST(source.clone());
        if print_ {
            println!("{:?}\n", output);
        }

        // Stage 3: Evaluate
        let evaluated = interpreter::evaluate(Box::new(output.clone()), &env);
        if print_ {
            println!("-------------------------- Runtime -------------------------------\n");
            println!("Runtime Value Debug: {:?}\n", evaluated);
        } else {
            println!("{:?}\n", evaluated);
        }
    }
}

