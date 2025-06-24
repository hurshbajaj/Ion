use std::any::Any;

use crate::ast::{self, BinExpr, Identifier, Nil, NumericLiteral, Program, Stmt};
use crate::scopes::Scope;
use crate::values::{BooleanVal, NilVal, NumericVal, RuntimeValue, RuntimeValueType};

#[derive(Debug)]
pub enum RuntimeValueServe<'a> {
    Owned(Box<dyn RuntimeValue>),
    Ref(&'a dyn RuntimeValue),
}

pub fn evaluate<'a>(ASTnode: Box<dyn Stmt>, scope: &'a Scope) -> RuntimeValueServe<'a> {
    match ASTnode.kind() {
        ast::NodeType::NumericLiteralNode => {
            let val = ASTnode
                .as_any()
                .downcast_ref::<NumericLiteral<f64>>()
                .unwrap()
                .value;
            let boxed = Box::new(NumericVal { value: val });
            return RuntimeValueServe::Owned(boxed);
        }
        ast::NodeType::Identifier => {
            return eval_identifier(ASTnode.as_any().downcast_ref::<Identifier>().unwrap(), scope)
        }
        ast::NodeType::Nil => {
            return RuntimeValueServe::Owned(Box::new(NilVal {}));
        }
        ast::NodeType::Bool => {
            return RuntimeValueServe::Owned(Box::new(BooleanVal{value: ASTnode.as_any().downcast_ref::<ast::Bool>().unwrap().value}))
        }
        ast::NodeType::BinOp => {
            return eval_bin_expr(ASTnode.as_any().downcast_ref::<BinExpr>().unwrap(), scope);
        }
        ast::NodeType::Program => {
            return eval_program(ASTnode, scope);
        }
        _ => {
            panic!("Can't evaluate AST node yet: {:?}", ASTnode);
        }
    }
}

fn eval_identifier<'a>(unwrap: &Identifier, scope: &'a Scope) -> RuntimeValueServe<'a> {
    RuntimeValueServe::Ref(scope.loopup(unwrap.symbol.clone()).as_ref())
}

fn eval_program<'a>(
    astnode: Box<dyn Stmt>,
    scope: &'a Scope,
) -> RuntimeValueServe<'a> {
    let mut last_eval: RuntimeValueServe<'a> = RuntimeValueServe::Owned(Box::new(NilVal {}));
    let program = astnode.as_any().downcast_ref::<Program>().unwrap();
    for stmt in &program.body {
        last_eval = evaluate(stmt.clone(), scope);
    }
    last_eval
}

fn eval_bin_expr<'a>(unwrap: &BinExpr, scope: &'a Scope) -> RuntimeValueServe<'a> {
    let lhs = evaluate(unwrap.left.clone(), scope);
    let rhs = evaluate(unwrap.right.clone(), scope);

    match (lhs, rhs) {
        (RuntimeValueServe::Owned(lhs_val), RuntimeValueServe::Owned(rhs_val)) => {
            if lhs_val.Type() == RuntimeValueType::Numeric
                && rhs_val.Type() == RuntimeValueType::Numeric
            {
                return eval_numeric_bin_expr(
                    RuntimeValueServe::Owned(lhs_val),
                    RuntimeValueServe::Owned(rhs_val),
                    unwrap.operator.as_str(),
                );
            }
        }
        (RuntimeValueServe::Ref(lhs_val), RuntimeValueServe::Owned(rhs_val)) => {
            if lhs_val.Type() == RuntimeValueType::Numeric
                && rhs_val.Type() == RuntimeValueType::Numeric
            {
                return eval_numeric_bin_expr(
                    RuntimeValueServe::Ref(lhs_val),
                    RuntimeValueServe::Owned(rhs_val),
                    unwrap.operator.as_str(),
                );
            }
        }
        (RuntimeValueServe::Owned(lhs_val), RuntimeValueServe::Ref(rhs_val)) => {
            if lhs_val.Type() == RuntimeValueType::Numeric
                && rhs_val.Type() == RuntimeValueType::Numeric
            {
                return eval_numeric_bin_expr(
                    RuntimeValueServe::Owned(lhs_val),
                    RuntimeValueServe::Ref(rhs_val),
                    unwrap.operator.as_str(),
                );
            }
        }
        (RuntimeValueServe::Ref(lhs_val), RuntimeValueServe::Ref(rhs_val)) => {
            if lhs_val.Type() == RuntimeValueType::Numeric
                && rhs_val.Type() == RuntimeValueType::Numeric
            {
                return eval_numeric_bin_expr(
                    RuntimeValueServe::Ref(lhs_val),
                    RuntimeValueServe::Ref(rhs_val),
                    unwrap.operator.as_str(),
                );
            }
        }
    }

    RuntimeValueServe::Owned(Box::new(NilVal {}))
}

fn eval_numeric_bin_expr<'a>(
    lhs_val: RuntimeValueServe<'a>,
    rhs_val: RuntimeValueServe<'a>,
    op: &str,
) -> RuntimeValueServe<'a> {
    // Extract values from the variants
    let lhs_num = match lhs_val {
        RuntimeValueServe::Owned(ref v) => v.as_any().downcast_ref::<NumericVal>().unwrap().value,
        RuntimeValueServe::Ref(v) => v.as_any().downcast_ref::<NumericVal>().unwrap().value,
    };

    let rhs_num = match rhs_val {
        RuntimeValueServe::Owned(ref v) => v.as_any().downcast_ref::<NumericVal>().unwrap().value,
        RuntimeValueServe::Ref(v) => v.as_any().downcast_ref::<NumericVal>().unwrap().value,
    };

    let result = match op {
        "+" => lhs_num + rhs_num,
        "-" => lhs_num - rhs_num,
        "*" => lhs_num * rhs_num,
        "/" => {
            if rhs_num == 0.0 {
                panic!("Division by zero");
            }
            lhs_num / rhs_num
        }
        _ => panic!("Improper tokenization error"),
    };

    RuntimeValueServe::Owned(Box::new(NumericVal { value: result }))
}

