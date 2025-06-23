#![feature(trait_upcasting)]

use std::fs;
mod lexer;
mod parser;
mod ast;

fn main() {
    let source = fs::read_to_string("code.io")
        .expect("Failed to read file 'code.io'");

    unsafe{
        let output = parser::prodAST(source);
        println!("{:?}", output);
        for x in output.body.iter() {
            println!("{:?}", x);
        }
    }

}
