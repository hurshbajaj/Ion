#![feature(trait_upcasting)]

use std::fs;
mod lexer;
mod parser;
mod ast;

mod runtime;
use runtime::*;

fn main() {
    let source = fs::read_to_string("code.io")
        .expect("Failed to read file 'code.io'");

    unsafe{
        let output = parser::prodAST(source);
        println!("Abstract Syntax Tree: {:?}", output);
        let evaluated = interpreter::evaluate(Box::new(output));
        println!("Runtime Value Debug: {:?}", evaluated);

    }

}
