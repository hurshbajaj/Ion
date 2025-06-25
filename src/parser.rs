use std::panic;
use std::str::FromStr;
use std::fmt::Debug;

use crate::{ast::*, lexer::{self, *}};
use num_traits::{Num, ToPrimitive, FromPrimitive};

static mut tokens: Vec<Token> = vec![];

pub unsafe fn prodAST(source_c: String) -> Program {
    tokens = tokenize(source_c);

    let mut program = Program{body: vec![]};

    while tokens.len() > 0{
        if tokens.first().is_some() {
            if tokens.first().unwrap().value_type == TokenType::EOF {
                break;
            }
        }

        program.body.push(parse_stmt());
    }

    program
}

unsafe fn parse_stmt() -> Box<dyn Stmt>{
    match tokens[0].value_type {
        TokenType::Let_k => {
            return parse_var_decl();
        },
        TokenType::Identifier => {
            if tokens.len() > 1 { //TODO CHANGE TO (2) B4 RELEASE
                if tokens[1].value_type == TokenType::Flag(Flags::Assign_f) {
                    return parse_var_asg();
                }else{
                    return parse_expr();
                }
            }else{
                panic!("Trailing Identifier...")
            }
        },
        _ => {
            return parse_expr();
        }
    }
}

unsafe fn parse_var_asg() -> Box<dyn Stmt> {
    let lhs = parse_expr();
    tokens.remove(0);
    if tokens.len() > 1{
        let rhs = parse_expr();
        end_stmt();
        return Box::new(VarAsg{lhs: lhs, rhs: rhs})
    }else{
        panic!("Incomplete Variable Assignment");
    }
}

unsafe fn parse_var_decl() -> Box<dyn Stmt> {
    tokens.remove(0); 
    let ident = tokens.remove(0);
    if ident.value_type != TokenType::Identifier {
        panic!("Missing Identifier");
    }

    let mut found_flags = vec![];

    while let TokenType::Flag(ref flag) = tokens[0].value_type {
        let flag = flag.clone(); // clone the flag so you keep ownership
        tokens.remove(0);
        found_flags.push(flag);
    }

    let mut value: Box<dyn Expr> = Box::new(Nil {});

    if found_flags.contains(&Flags::Assign_f) {
        value = parse_expr(); 
    }

    end_stmt();
    Box::new(VarDeclaration {
        identifier: ident.value,
        flags: found_flags,
        value,
    })
}

unsafe fn parse_expr() -> Box<dyn Expr>{
    return parse_object_literal_expr();
}

unsafe fn parse_object_literal_expr() -> Box<dyn Expr> {
   if tokens[0].value_type != TokenType::Identifier || tokens[1].value_type != TokenType::LeftCurly{
        return parse_object_expr();
   }
   tokens.drain(..2);
   let mut props = vec![];
   while tokens[0].value_type == TokenType::Identifier {
        let key = tokens.remove(0).value;
        tokens.remove(0);
        let value = parse_expr();
        tokens.remove(0);
        props.push(PropertyLiteral{key, value});
   }
   tokens.remove(0);
   return Box::new(ObjectLiteral{properties: props})
}

unsafe fn parse_object_expr() -> Box<dyn Expr> {
   if tokens[0].value_type != TokenType::LeftCurly{
        return parse_additive_expr();
   }
   tokens.remove(0);
   let mut props = vec![];
   while tokens[0].value_type == TokenType::Identifier {
        let key = tokens.remove(0).value;
        tokens.remove(0);
        let value = get_attr(Some(tokens.remove(0).value.as_str())).unwrap_or_else(||{
            panic!("Incorrect type attr provided for object key")
        });
        tokens.remove(0);
        props.push(Property{key, value});
   }
   tokens.remove(0);
   return Box::new(Object{properties: props})
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

unsafe fn parse_prim_expr() -> Box<dyn Expr> {
    let TkType = tokens[0].clone();

    match TkType.value_type {
        TokenType::Identifier => {
            Box::new(Identifier { symbol: tokens.remove(0).value })
        }
        TokenType::Number => {
            Box::new(NumericLiteral::<f64> {
                value: parse_num::<f64>(tokens.remove(0).value.as_str()),
            })
        }
        TokenType::BinOp => {
            if TkType.value == "-" {
                tokens.remove(0);
                tokens.insert(0, Token{value: "*".to_string(), value_type: TokenType::BinOp});
                tokens.insert(0, Token{value: "-1".to_string(), value_type: TokenType::Number});
                return parse_expr()
            }else{
                panic!("Trailing Binary Operator")
            }
        }
        TokenType::LeftParen => {
            tokens.remove(0);
            let value = parse_expr();
            if tokens[0].value != ")" {
                panic!("Missing Closing Paren");
            }
            tokens.remove(0);
            return value;
        }
        TokenType::Nil_k => {
            tokens.remove(0);
            Box::new(Nil {})
        }
        TokenType::Bool_true_t => {
            tokens.remove(0);
            Box::new(Bool { value: true })
        }
        TokenType::Bool_false_t => {
            tokens.remove(0);
            Box::new(Bool { value: false })
        }
        _ => {
            panic!("Unexpected Token: {:?}; Cannot be parsed as an expression", TkType)
        }
    }
}


pub fn parse_num<T>(s: &str) -> T
where
    T: Num + FromStr + Copy,
    <T as FromStr>::Err: Debug
{
    T::from_str(s).unwrap()
}

unsafe fn end_stmt(){
    if tokens[0].value_type == TokenType::Semicolon{
        tokens.remove(0);
    }else{
        panic!("Statement must end with a ;");
    }
}

