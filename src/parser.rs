use std::panic;
use std::str::FromStr;
use std::fmt::Debug;

use crate::{ast::*, lexer::*};
use num_traits::{Num, ToPrimitive, FromPrimitive};

static mut tokens: Vec<Token> = vec![];

pub unsafe fn prodAST(source_c: String) -> Program {
    tokens = tokenize(source_c);

    let mut program = Program{body: vec![]};

    while tokens.len() > 0{
        program.body.push(parse_stmt());
    }

    program
}

unsafe fn parse_stmt() -> Box<dyn Stmt>{
   return parse_expr(); 
}

unsafe fn parse_expr() -> Box<dyn Expr>{
    return parse_additive_expr();
}

unsafe fn parse_additive_expr() -> Box<dyn Expr> {
    let mut left = parse_multiplicative_expr();

    while !tokens.is_empty() && (tokens[0].clone().value == "+" || tokens[0].clone().value == "-") {
        let op = tokens.remove(0).value;
        let right = parse_multiplicative_expr();
        left = Box::new(BinExpr {
            left,
            right,
            operator: op,
        });
    }

    left
}

unsafe fn parse_multiplicative_expr() -> Box<dyn Expr> {
    let mut left = parse_prim_expr();

    while !tokens.is_empty() && (tokens[0].clone().value == "*" || tokens[0].clone().value == "/") {
        let op = tokens.remove(0).value;
        let right = parse_prim_expr();
        left = Box::new(BinExpr {
            left,
            right,
            operator: op,
        });
    }

    left
}

unsafe fn parse_prim_expr() -> Box<dyn Expr>{
    let TkType = tokens[0].clone();

    match TkType.value_type{
        TokenType::Identifier => {
            Box::new(Identifier{symbol: tokens.remove(0).value})
        },
        TokenType::Number => {
            Box::new(NumericLiteral::<f64>{value: parse_num::<f64>( tokens.remove(0).value.as_str() )})
        },
        TokenType::LeftParen => {
            tokens.remove(0);
            let value = parse_expr();
            if tokens[0].value != ")"{
                panic!("Missing Closing Paren");
            }
            tokens.remove(0);
            return value;
        },
        TokenType::Nil_k => {
            tokens.remove(0);
            Box::new(Nil{})
        },
        _ => { panic!("Unexpected Token: {:?}", TkType) }
    }
}

pub fn parse_num<T>(s: &str) -> T
where
    T: Num + FromStr + Copy,
    <T as FromStr>::Err: Debug
{
    T::from_str(s).unwrap()
}

