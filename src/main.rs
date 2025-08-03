use std::cell::RefCell;
use std::fs;
mod lexer;
mod parser;
mod ast;

mod runtime;
use runtime::*;

use crate::scopes::init;

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

        let evaluated = interpreter::evaluate(Box::new(output.clone()), Box::leak(Box::new(RefCell::new(init()))));
        if PRINT_ {
            println!("-------------------------- Runtime -------------------------------\n");
            println!("Runtime Value Debug: {:?}\n", evaluated);
        } 
    }
}

/*
use std::cell::RefCell;
use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod runtime;

use runtime::*;
use crate::scopes::init;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_mode = args.get(1);

    match file_mode {
        Some(arg) if arg == "repl" => {
            // TODO: Implement REPL mode if you haven't already.
            println!("🚧 REPL mode not yet implemented.");
        }
        Some(file_path) => {
            if !file_path.ends_with(".io") {
                eprintln!("❌ Error: Only '.io' files are allowed.");
                std::process::exit(1);
            }

            let source = fs::read_to_string(file_path)
                .unwrap_or_else(|_| {
                    eprintln!("❌ Error reading file '{}'", file_path);
                    std::process::exit(1);
                });

            unsafe {
                if PRINT_ {
                    println!("\n-------------------------- Original -------------------------------\n");
                    println!("{}\n", source);
                }

                let output = parser::prod_ast(source.clone());
                if PRINT_ {
                    println!("{:?}\n", output);
                }

                let evaluated = interpreter::evaluate(
                    Box::new(output.clone()),
                    Box::leak(Box::new(RefCell::new(init()))),
                );

                if PRINT_ {
                    println!("-------------------------- Runtime -------------------------------\n");
                    println!("Runtime Value Debug: {:?}\n", evaluated);
                }
            }
        }
        None => {
            eprintln!("❌ No input provided. Use 'make run file=filename.io' or 'make run' for REPL.");
            std::process::exit(1);
        }
    }
}
*/
