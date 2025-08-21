use std::{fmt::Debug, str::FromStr};

use crate::{ast::*, lexer::*};
use num_traits::Num;

static mut TOKENS: Vec<Token> = vec![];

pub unsafe fn prod_ast(source_c: String) -> Program {
    TOKENS = tokenize(source_c);

    let mut program = Program{body: vec![]};

    while TOKENS.len() > 0{
        if TOKENS.first().is_some() {
            if TOKENS.first().unwrap().value_type == TokenType::EOF {
                break;
            }
        }

        program.body.push(parse_to_stmt());
    }

    program
}

unsafe fn parse_to_stmt() -> Box<dyn Stmt>{
    match TOKENS[0].value_type {
        TokenType::Let_k => {
            let rt = parse_var_decl();
            end_stmt();
            return rt;
        },
        TokenType::Identifier => {
            if TOKENS[1].value_type == TokenType::Flag(Flags::Assign_f) {
                let rt = parse_var_asg();
                end_stmt();
                return rt;
            }else{
                let rt = parse_expr();
                end_stmt();
                return rt;
            }

        },
        _ => {
            let rt =  parse_expr();
            end_stmt();
            return rt;
        }
    }
}

unsafe fn parse_var_asg() -> Box<dyn Stmt> {
    let lhs = parse_expr();
    TOKENS.remove(0);
    if TOKENS.len() > 1{
        let rhs = parse_expr();
        return Box::new(VarAsg{lhs: lhs, rhs: rhs})
    }else{
        panic!("Incomplete Variable Assignment");
    }
}

unsafe fn parse_var_decl() -> Box<dyn Stmt> {
    TOKENS.remove(0); 
    let ident = TOKENS.remove(0);
    if ident.value_type != TokenType::Identifier {
        panic!("Missing Identifier");
    }

    let mut found_flags = vec![];

    while let TokenType::Flag(ref flag) = TOKENS[0].value_type {
        let flag = flag.clone(); // clone the flag so you keep ownership
        TOKENS.remove(0);
        found_flags.push(flag);
    }

    let mut value: Box<dyn Expr> = Box::new(Nil {});

    if found_flags.contains(&Flags::Assign_f) {
        value = parse_expr(); 
    }

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
   if TOKENS[0].value_type != TokenType::Identifier || TOKENS[1].value_type != TokenType::LeftCurly{
        return parse_object_expr();
   }
   TOKENS.remove(0);
   let mut props = vec![];
   while TOKENS[0].value_type == TokenType::Identifier {
        let key = TOKENS.remove(0).value;
        TOKENS.remove(0);
        let value = parse_expr();
        TOKENS.remove(0);
        props.push(PropertyLiteral{key, value});
   }
   TOKENS.remove(0);
   return Box::new(ObjectLiteral{properties: props})
}

unsafe fn parse_object_expr() -> Box<dyn Expr> {
   if TOKENS[0].value_type != TokenType::obj_struct_k{
        return parse_array_literal_expr();
   }
   TOKENS.remove(0);
   TOKENS.remove(0);
   let mut props = vec![];
   while TOKENS[0].value_type == TokenType::Identifier {
        let key = TOKENS.remove(0).value;
        TOKENS.remove(0);
        let value = get_attr(Some(TOKENS.remove(0).value.as_str())).unwrap_or_else(||{
            panic!("Incorrect type attr provided for object key")
        });
        TOKENS.remove(0);
        props.push(Property{key, value});
   }
   TOKENS.remove(0);
   return Box::new(Object{properties: props})
}

unsafe fn parse_array_literal_expr() -> Box<dyn Expr> {
   if TOKENS[0].value_type != TokenType::LeftBrace{
        return parse_array_expr();
   }
   TOKENS.remove(0);
   let mut entries = vec![];
   while TOKENS[0].value_type != TokenType::RightBrace {
       entries.push(parse_expr());
       if TOKENS[0].value_type == TokenType::RightBrace {break;};
       TOKENS.remove(0);
   }
   TOKENS.remove(0);
   return Box::new(ArrayLiteral{entries})
}


unsafe fn parse_array_expr() -> Box<dyn Expr> { // [numeric ; nil ; 10;]
   if TOKENS[0].value_type != TokenType::arr_struct_k{
        return parse_additive_expr();
   }
   TOKENS.remove(0);
   TOKENS.remove(0);
   let attr_shell = TOKENS.remove(0);
   let attr = get_attr(Some(attr_shell.value.as_str()));
   TOKENS.remove(0);
   let complex_attr_shell = TOKENS.remove(0);
   let mut complex_attr = None;
   if complex_attr_shell.value_type != TokenType::Nil_k{
        complex_attr = Some(complex_attr_shell.value);
   }
   TOKENS.remove(0);
   let length = usize::from_str( &(TOKENS.remove(0).value));
   TOKENS.remove(0);
   TOKENS.remove(0);
   return Box::new(Array{attr: attr.unwrap(), complex_attr, length: length.unwrap()});
}

unsafe fn parse_fn_struct() -> Box<dyn Expr> {
    if TOKENS[0].value_type != TokenType::fn_struct_k { 
        return parse_call_mem_expr();
    }

    TOKENS.remove(0);
    TOKENS.remove(0);
    let mut params = vec![];
    while TOKENS[0].value_type == TokenType::Identifier {
        let param = TOKENS.remove(0).value;
        TOKENS.remove(0);
        let param_type = get_attr(Some(TOKENS.remove(0).value.as_str())).unwrap_or_else(|| {
            panic!("Incorrect type attr provided as function-struct param-type");
        });
        TOKENS.remove(0);
        params.push(Param{param, param_type}); 
    }
    TOKENS.remove(0);
    TOKENS.remove(0);
    return Box::new(FnStruct{params, ret_type: get_attr(Some(TOKENS.remove(0).value.as_str())).unwrap_or_else(|| {
            panic!("Incorrect type attr provided as function-struct return-type");
        })});
}

unsafe fn parse_additive_expr() -> Box<dyn Expr> {
    let mut left = parse_multiplicative_expr();

    while !TOKENS.is_empty() && (TOKENS[0].clone().value == "+" || TOKENS[0].clone().value == "-") {
        let op = TOKENS.remove(0).value;
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
    let mut left = parse_fn_struct(); //ideally, call parse fn struct

    while !TOKENS.is_empty() && (TOKENS[0].clone().value == "*" || TOKENS[0].clone().value == "/") {
        let op = TOKENS.remove(0).value;
        let right = parse_prim_expr();
        left = Box::new(BinExpr {
            left,
            right,
            operator: op,
        });
    }

    left
}

/*
unsafe fn parse_call_mem_expr() -> Box<dyn Expr>{
    let member = parse_mem_expr();
    if TOKENS[0].value_type == TokenType::LeftParen {
        return parse_call_expr(member);
    }
    return member;
}

unsafe fn parse_mem_expr() -> Box<dyn Expr> {
    let mut obj = parse_prim_expr();
    while TOKENS[0].value_type == TokenType::Dot {
        TOKENS.remove(0);
        let prop: Box<dyn Expr>;
        prop = parse_prim_expr();
        if !prop.as_any().downcast_ref::<Identifier>().is_some() {
            panic!("Right hand side of the dot operator must be an Identifier");
        }
        obj = Box::new(MemberExpr{obj, prop});
    }

    return obj;
}
*/
unsafe fn parse_call_array_mem_expr() -> Box<dyn Expr>{
    todo!();
}
unsafe fn parse_call_mem_expr() -> Box<dyn Expr>{
    let member = parse_mem_expr(parse_prim_expr());
    if TOKENS[0].value_type == TokenType::LeftParen {
        return parse_call_expr(member);
    }
    return member;
}

unsafe fn parse_mem_expr(mut at: Box<dyn Expr>) -> Box<dyn Expr> {
    if TOKENS[0].value_type == TokenType::Dot {
        TOKENS.remove(0);
        let prop: Box<dyn Expr>;
        prop = parse_prim_expr();
        if !prop.as_any().downcast_ref::<Identifier>().is_some() {
            panic!("Right hand side of the dot operator must be an Identifier");
        }
        at = parse_mem_expr(Box::new(MemberExpr{obj: at, prop}) );
    }

    return at;
}

unsafe fn parse_call_expr(call_to: Box<dyn Expr>) -> Box<dyn Expr> { // accounts for xyz()()()...
                                                                     // as well
    let mut call_expr = Box::new(CallExpr{
        call_to,
        args: parse_args()
    });
    if TOKENS[0].value_type == TokenType::LeftParen {
        call_expr = Box::new(parse_call_expr(call_expr).as_any().downcast_ref::<CallExpr>().unwrap().clone());
    }
    return parse_mem_expr( call_expr );
}   

unsafe fn parse_args() -> Vec<Box<dyn Expr>> { 
    TOKENS.remove(0);
    let mut args = vec![];
    if TOKENS[0].value_type == TokenType::RightParen {}
    else{
        args = parse_args_list();
    }
    if TOKENS[0].value_type != TokenType::RightParen {
        panic!("Missing Closing Paren");
    }
    TOKENS.remove(0);
    return args;

}

unsafe fn parse_args_list() -> Vec<Box<dyn Expr>> {
    let mut args = vec![parse_expr()];
    while TOKENS[0].value_type == TokenType::Comma {
        TOKENS.remove(0);
        args.push(parse_expr());
    }
    return args;
}

unsafe fn parse_prim_expr() -> Box<dyn Expr> {
    let TkType = TOKENS[0].clone();

    match TkType.value_type {
        TokenType::Identifier => {
            Box::new(Identifier { symbol: TOKENS.remove(0).value })
        }
        TokenType::Number => {
            Box::new(NumericLiteral::<f64> {
                value: parse_num::<f64>(TOKENS.remove(0).value.as_str()),
            })
        }
        TokenType::BinOp => {
            if TkType.value == "-" {
                TOKENS.remove(0);
                TOKENS.insert(0, Token{value: "*".to_string(), value_type: TokenType::BinOp});
                TOKENS.insert(0, Token{value: "-1".to_string(), value_type: TokenType::Number});
                return parse_expr()
            }else{
                panic!("Trailing Binary Operator")
            }
        }
        TokenType::LeftParen => {
            TOKENS.remove(0);
            let value = parse_expr();
            if TOKENS[0].value != ")" {
                panic!("Missing Closing Paren");
            }
            TOKENS.remove(0);
            return value;
        }
        TokenType::Nil_k => {
            TOKENS.remove(0);
            Box::new(Nil {})
        }
        TokenType::Bool_true_t => {
            TOKENS.remove(0);
            Box::new(Bool { value: true })
        }
        TokenType::Bool_false_t => {
            TOKENS.remove(0);
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
    if TOKENS[0].value_type == TokenType::Semicolon{
        TOKENS.remove(0);
    }else{
        panic!("Statement must end with a ;");
    }
}

