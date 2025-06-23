use std::any::Any;

use crate::ast::{self, BinExpr, Nil, NumericLiteral, Program, Stmt};
use crate::values::{NilVal, NumericVal, RuntimeValue, RuntimeValueType};

pub unsafe fn evaluate (ASTnode: Box<dyn Stmt>) -> Box<dyn RuntimeValue>{
    match ASTnode.kind() {
        ast::NodeType::NumericLiteralNode => {
            return Box::new(NumericVal{value: ASTnode.as_any().downcast_ref::<NumericLiteral<f64>>().unwrap().value})
        },
        ast::NodeType::Nil => {
            return Box::new(NilVal{})
        },
        ast::NodeType::BinOp => {
            return eval_bin_expr(ASTnode.as_any().downcast_ref::<BinExpr>().unwrap())
        },
        ast::NodeType::Program => {
            return eval_program(ASTnode)
        },
        _ => {
            panic!("Can't evaluate AST node yet: {:?}", ASTnode);
        }
    }
}

unsafe fn eval_program(astnode: Box<dyn Stmt>) -> Box<dyn RuntimeValue> {
    let mut last_eval: Box<dyn RuntimeValue> = Box::new(NilVal{});
    let program = astnode.as_any().downcast_ref::<Program>().unwrap();
    for stmt in &program.body{
        last_eval = evaluate(stmt.clone());
    }
    return last_eval;
}

unsafe fn eval_bin_expr(unwrap: &BinExpr) -> Box<dyn RuntimeValue> {
    let lhs = evaluate(unwrap.left.clone());
    let rhs = evaluate(unwrap.right.clone());

    if lhs.Type() == RuntimeValueType::Numeric && rhs.Type() == RuntimeValueType::Numeric {
        return eval_numeric_bin_expr(unwrap)
    }

    return Box::new(NilVal{})
}

fn eval_numeric_bin_expr(unwrap: &BinExpr) -> Box<dyn RuntimeValue> {
    let mut result = 0f64;
    match unwrap.operator.as_str() {
        "+" => {
            result = unwrap.left.as_any().downcast_ref::<NumericLiteral>().unwrap().value + unwrap.right.as_any().downcast_ref::<NumericLiteral>().unwrap().value;
        },
        "-" => {
            result = unwrap.left.as_any().downcast_ref::<NumericLiteral>().unwrap().value - unwrap.right.as_any().downcast_ref::<NumericLiteral>().unwrap().value;
        },
        "*" => {
            result = unwrap.left.as_any().downcast_ref::<NumericLiteral>().unwrap().value * unwrap.right.as_any().downcast_ref::<NumericLiteral>().unwrap().value;
        },
        "/" => {
            if unwrap.right.as_any().downcast_ref::<NumericLiteral>().unwrap().value == 0f64 {panic!("Division By Zero")}
            result = unwrap.left.as_any().downcast_ref::<NumericLiteral>().unwrap().value / unwrap.right.as_any().downcast_ref::<NumericLiteral>().unwrap().value;
        },
        _ => {panic!("Improper Tokenization Error")}
    }
    Box::new(NumericVal{value: result})
}
