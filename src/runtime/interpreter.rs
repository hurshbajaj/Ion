use std::any::Any;
use std::cell::{Ref, RefCell};

use crate::ast::{self, BinExpr, Identifier, Nil, NumericLiteral, Program, Stmt, VarDeclaration};
use crate::scopes::Scope;
use crate::values::{BooleanVal, NilVal, NumericVal, RuntimeValue, RuntimeValueType, StmtExecS};

#[derive(Debug)]
pub enum RuntimeValueServe<'a> {
    Owned(Box<dyn RuntimeValue>),
    Ref(Ref<'a, Box<dyn RuntimeValue>>),
}


pub fn evaluate<'a>(ASTnode: Box<dyn Stmt>, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    match ASTnode.kind() {
        ast::NodeType::NumericLiteralNode => {
            let val = ASTnode
                .as_any()
                .downcast_ref::<NumericLiteral<f64>>()
                .unwrap()
                .value;
            RuntimeValueServe::Owned(Box::new(NumericVal { value: val }))
        }
        ast::NodeType::Identifier => {
            eval_identifier(ASTnode.as_any().downcast_ref::<Identifier>().unwrap(), scope)

        }
        ast::NodeType::Nil => RuntimeValueServe::Owned(Box::new(NilVal {})),
        ast::NodeType::Bool => RuntimeValueServe::Owned(Box::new(BooleanVal {
            value: ASTnode
                .as_any()
                .downcast_ref::<ast::Bool>()
                .unwrap()
                .value,
        })),
        ast::NodeType::BinOp => {
            eval_bin_expr(ASTnode.as_any().downcast_ref::<BinExpr>().unwrap(), scope)
        }
        ast::NodeType::Program => eval_program(ASTnode, scope),
        ast::NodeType::VarDecl => {
            eval_var_decl(ASTnode.as_any().downcast_ref::<VarDeclaration>().unwrap(), scope)
        }
        _ => panic!("Can't evaluate AST node yet: {:?}", ASTnode),
    }
}

fn eval_var_decl<'a>(unwrap: &VarDeclaration, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    let evaluated = evaluate(unwrap.value.clone(), scope);

    let val_to_store = match evaluated {
        RuntimeValueServe::Owned(val) => val,
        RuntimeValueServe::Ref(val) => val.clone_box(),
    };

    scope
        .borrow_mut()
        .var_decl(unwrap.identifier.clone(), val_to_store);

    RuntimeValueServe::Owned(Box::new(StmtExecS {}))
}

fn eval_identifier<'a>(
    unwrap: &Identifier,
    scope: &'a RefCell<Scope>,
) -> RuntimeValueServe<'a> {
    let scope_borrow = scope.borrow();
    let val_ref = Ref::map(scope_borrow, |s| {
        s.loopup(unwrap.symbol.clone())
    });
    RuntimeValueServe::Ref(val_ref)
}


fn eval_program<'a>(astnode: Box<dyn Stmt>, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    let program = astnode.as_any().downcast_ref::<Program>().unwrap();

    if program.body.is_empty() {
        return RuntimeValueServe::Owned(Box::new(NilVal {}));
    }

    for stmt in &program.body[..program.body.len() - 1] {
        let _ = evaluate(stmt.clone(), scope);
    }

    evaluate(program.body.last().unwrap().clone(), scope)
}

fn eval_bin_expr<'a>(unwrap: &BinExpr, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
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

