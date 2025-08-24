use std::cell::RefCell;
use std::fs;
mod lexer;
mod parser;
mod ast;

mod runtime;
use runtime::*;

use crate::scopes::init;

use pprof::ProfilerGuard;

fn main() {
    let source = fs::read_to_string("main.io")
        .expect("Failed to read file 'main.io'");
    unsafe {
        if PRINT_ {
            println!("\n-------------------------- Original -------------------------------\n");
            println!("{}\n", source);
        }

        let output = parser::prod_ast(source.clone());
        if PRINT_ {
            println!("{:?}\n", output);
        }
        println!("----------------------------Logs----------------------------------");

        let evaluated = interpreter::evaluate(Box::new(output.clone()), Box::leak(Box::new(RefCell::new(init()))));
        if PRINT_ {
            println!("-------------------------- Runtime -------------------------------\n");
            println!("Runtime Value Debug: {:?}\n", evaluated);
        } 
    }
}

