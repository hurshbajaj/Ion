use std::any::Any;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::thread::scope;

use crate::ast::{self, BinExpr, Identifier, Nil, NodeType, NumericLiteral, Object, ObjectLiteral, Program, Stmt, VarAsg, VarDeclaration};
use crate::lexer::{Attr, TokenType};
use crate::scopes::Scope;
use crate::values::{BooleanVal, NilVal, NumericVal, ObjectLiteralVal, ObjectVal, RuntimeValue, RuntimeValueType, StmtExecS};

#[derive(Debug)]
pub enum RuntimeValueServe<'a> {
    Owned(Box<dyn RuntimeValue>),
    Ref(Ref<'a, Box<dyn RuntimeValue>>),
}

impl<'a> Clone for RuntimeValueServe<'a> {
    fn clone(&self) -> Self {
        match self {
            RuntimeValueServe::Owned(b) => RuntimeValueServe::Owned(b.clone()),
            RuntimeValueServe::Ref(r) => RuntimeValueServe::Owned(r.clone_box()), 
        }
    }
}


pub fn evaluate<'a>(ASTnode: Box<dyn Stmt>, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    match ASTnode.kind() {
        ast::NodeType::NumericLiteralNode => {
            let val = ASTnode
                .as_any()
                .downcast_ref::<NumericLiteral<f64>>() // Still parsing as f64
                .unwrap()
                .value;

            let minimized = minimize_numeric(val); // <- call to logic you built

            let boxed: Box<dyn RuntimeValue> = match minimized {
                MinimizedNumeric::I8(v) => Box::new(v),
                MinimizedNumeric::I16(v) => Box::new(v),
                MinimizedNumeric::I32(v) => Box::new(v),
                MinimizedNumeric::I64(v) => Box::new(v),
                MinimizedNumeric::U8(v) => Box::new(v),
                MinimizedNumeric::U16(v) => Box::new(v),
                MinimizedNumeric::U32(v) => Box::new(v),
                MinimizedNumeric::U64(v) => Box::new(v),
                MinimizedNumeric::F32(v) => Box::new(v),
                MinimizedNumeric::F64(v) => Box::new(v),
            };

            RuntimeValueServe::Owned(boxed)
        }

        ast::NodeType::Identifier => {
            eval_identifier(ASTnode.as_any().downcast_ref::<Identifier>().unwrap(), scope)

        }
        ast::NodeType::Nil => RuntimeValueServe::Owned(Box::new(NilVal {})),
        ast::NodeType::Bool => RuntimeValueServe::Owned(Box::new(BooleanVal {
            val: ASTnode
                .as_any()
                .downcast_ref::<ast::Bool>()
                .unwrap()
                .value,
        })),
        ast::NodeType::BinOp => {
            eval_bin_expr(ASTnode.as_any().downcast_ref::<BinExpr>().unwrap(), scope)
        },
        ast::NodeType::Object => {
            eval_obj_expr(ASTnode.as_any().downcast_ref::<Object>().unwrap(), scope)
        },
        ast::NodeType::ObjectLiteral => {
            eval_obj_literal_expr(ASTnode.as_any().downcast_ref::<ObjectLiteral>().unwrap(), scope)
        }
        ast::NodeType::Program => eval_program(ASTnode, scope),
        ast::NodeType::VarDecl => {
            eval_var_decl(ASTnode.as_any().downcast_ref::<VarDeclaration>().unwrap(), scope)
        },
        ast::NodeType::VarAsg => {
            eval_var_asg(ASTnode.as_any().downcast_ref::<VarAsg>().unwrap(), scope)
        },
        _ => panic!("Can't evaluate AST node yet: {:?}", ASTnode),
    }
}

fn eval_obj_literal_expr<'a>(unwrap: &ObjectLiteral, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    let mut object = ObjectLiteralVal { properties: HashMap::new() };
    for prop in &unwrap.properties {
        let mut val = evaluate(  prop.value.clone() , scope); //LET IT BE FU*KING CLONED IVE THOUGHT ABT THIS U FU*KING ******
        if let RuntimeValueServe::Owned(v) = val{
            object.properties.insert(prop.key.clone(), v);
        }else if let RuntimeValueServe::Ref(v) = val{
            object.properties.insert(prop.key.clone(), v.clone_box());
        }

    }
    RuntimeValueServe::Owned(Box::new(object))
}

fn eval_obj_expr<'a>(unwrap: &Object, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    let mut object = ObjectVal { properties: HashMap::new() };
    for prop in &unwrap.properties {
        let mut val = prop.value.clone(); //LET IT BE FU*KING CLONED IVE THOUGHT ABT THIS U FU*KING ******

        object.properties.insert(prop.key.clone(), val);
    }
    RuntimeValueServe::Owned(Box::new(object))
}

pub fn eval_var_asg<'a>(unwrap: &VarAsg, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    if unwrap.lhs.kind() != NodeType::Identifier {
        panic!("Can't assign value to parsed Expression");
    }

    let lhs_refined = unwrap.lhs.as_any().downcast_ref::<Identifier>().unwrap();
    let wrapped_rhs = evaluate(unwrap.rhs.clone(), scope);

    let refined_rhs = match wrapped_rhs {
        RuntimeValueServe::Owned(val) => val,
        RuntimeValueServe::Ref(val) => val.clone_box(),
    };

    let scope_refined = scope.borrow().clone();
    let _ = scope_refined.resolve(&lhs_refined.symbol);

    let f_flag = scope_refined.loopup_flags(lhs_refined.symbol.clone()).iter().find_map(|token_type| {
        if let crate::lexer::Flags::Struct_f(attr) = token_type {
            Some(attr.clone())
        } else {
            None
        }
    }).unwrap_or_else(|| panic!("Missing flag <structure> not found in Associated Variable Flags"));

    let complex_t: Option<Identifier> = scope_refined.loopup_flags(lhs_refined.symbol.clone()).iter().find_map(|token_type| {
        if let crate::lexer::Flags::Complex_f(Attr::Complex(attr)) = token_type {
            Some(Identifier { symbol: attr.clone() })
        } else {
            None
        }
    });

    static_type_check(refined_rhs.clone(), f_flag, complex_t, scope);

    scope
        .borrow_mut()
        .var_assign(lhs_refined.symbol.clone(), refined_rhs);

    RuntimeValueServe::Owned(Box::new(StmtExecS {}))
}

pub fn eval_var_decl<'a>(unwrap: &VarDeclaration, scope: &'a RefCell<Scope>) -> RuntimeValueServe<'a> {
    let evaluated = evaluate(unwrap.value.clone(), scope);

    let val_to_store = match evaluated {
        RuntimeValueServe::Owned(val) => val,
        RuntimeValueServe::Ref(val) => val.clone_box(),
    };

    let f_flag = unwrap.flags.iter().find_map(|token_type| {
        if let crate::lexer::Flags::Struct_f(attr) = token_type {
            Some(attr.clone())
        } else {
            None
        }
    }).unwrap_or_else(|| panic!("Missing flag <structure>"));

    let complex_t: Option<Identifier> = unwrap.flags.iter().find_map(|token_type| {
        if let crate::lexer::Flags::Complex_f(Attr::Complex(attr)) = token_type {
            Some(Identifier { symbol: attr.clone() })
        } else {
            None
        }
    });

    static_type_check(val_to_store.clone(), f_flag, complex_t, scope);

    scope
        .borrow_mut()
        .var_decl(unwrap.identifier.clone(), val_to_store, unwrap.flags.clone());

    RuntimeValueServe::Owned(Box::new(StmtExecS {}))
}


#[derive(Debug)]
pub enum MinimizedNumeric {
    // Signed integers
    I8(NumericVal<i8>),
    I16(NumericVal<i16>),
    I32(NumericVal<i32>),
    I64(NumericVal<i64>),

    // Unsigned integers
    U8(NumericVal<u8>),
    U16(NumericVal<u16>),
    U32(NumericVal<u32>),
    U64(NumericVal<u64>),

    // Floats
    F32(NumericVal<f32>),
    F64(NumericVal<f64>),
}

pub fn minimize_numeric(value: f64) -> MinimizedNumeric {
    if value.fract() == 0.0 {
        if value >= 0.0 {
            let int_val = value as u64;

            if int_val <= u8::MAX as u64 {
                MinimizedNumeric::U8(NumericVal { value: int_val as u8 })
            } else if int_val <= u16::MAX as u64 {
                MinimizedNumeric::U16(NumericVal { value: int_val as u16 })
            } else if int_val <= u32::MAX as u64 {
                MinimizedNumeric::U32(NumericVal { value: int_val as u32 })
            } else {
                MinimizedNumeric::U64(NumericVal { value: int_val })
            }
        } else {
            let int_val = value as i64;

            if int_val >= i8::MIN as i64 && int_val <= i8::MAX as i64 {
                MinimizedNumeric::I8(NumericVal { value: int_val as i8 })
            } else if int_val >= i16::MIN as i64 && int_val <= i16::MAX as i64 {
                MinimizedNumeric::I16(NumericVal { value: int_val as i16 })
            } else if int_val >= i32::MIN as i64 && int_val <= i32::MAX as i64 {
                MinimizedNumeric::I32(NumericVal { value: int_val as i32 })
            } else {
                MinimizedNumeric::I64(NumericVal { value: int_val })
            }
        }
    } else {
        let as_f32 = value as f32;
        if as_f32 as f64 == value {
            MinimizedNumeric::F32(NumericVal { value: as_f32 })
        } else {
            MinimizedNumeric::F64(NumericVal { value })
        }
    }
}

fn eval_identifier<'a>( unwrap: &Identifier, scope: &'a RefCell<Scope> ) -> RuntimeValueServe<'a> {
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
                    scope
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
                    scope
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
                    scope
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
                    scope
                );
            }
        }
    }

    RuntimeValueServe::Owned(Box::new(NilVal {}))
}

macro_rules! extract_numeric {
    ($any:expr, $cast_to:ty, [$($ty:ty),*]) => {
        $(
            if let Some(n) = $any.downcast_ref::<NumericVal<$ty>>() {
                let val: $cast_to = n.value as $cast_to;
                return val;
            }
        )*
    };
}


fn extract_as_i64(val: RuntimeValueServe) -> i64 {
    match val {
        RuntimeValueServe::Owned(v) => {
            let any = v.as_any();
            extract_numeric!(any, i64, [u8, u16, u32, u64, i8, i16, i32, i64]);
            panic!("Expected integer-compatible value for op");
        }
        _ => panic!("Interpreter Error: Numeric value expression cannot be Ref"),
    }
}
fn extract_as_f64(val: RuntimeValueServe) -> f64 {
    match val {
        RuntimeValueServe::Owned(v) => {
            let any = v.as_any();
            extract_numeric!(any, f64, [f64, f32, u8, u16, u32, u64, i8, i16, i32, i64]);
            panic!("Expected numeric value for f64 cast");
        }
        _ => panic!("Interpreter Error: Numeric value expression cannot be Ref"),
    }
}
fn is_float(val: &RuntimeValueServe) -> bool {
    match val {
        RuntimeValueServe::Owned(v) => {
            let any = v.as_any();
            any.is::<NumericVal<f64>>() || any.is::<NumericVal<f32>>()
        }
        _ => false,
    }
}

pub fn eval_numeric_bin_expr<'a>(
    lhs_val: RuntimeValueServe<'a>,
    rhs_val: RuntimeValueServe<'a>,
    op: &str,
    scope: &'a RefCell<Scope>,
) -> RuntimeValueServe<'a> {

    let should_use_float = is_float(&lhs_val) || is_float(&rhs_val);

    let result = if should_use_float {
        let lhs = extract_as_f64(lhs_val);
        let rhs = extract_as_f64(rhs_val);
        match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => {
                if rhs == 0.0 {
                    panic!("Division by zero");
                }
                lhs / rhs
            }
            _ => panic!("Invalid operator: {}", op),
        }
    } else {
        let lhs = extract_as_i64(lhs_val) as f64;
        let rhs = extract_as_i64(rhs_val) as f64;
        match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => {
                if rhs == 0.0 {
                    panic!("Division by zero");
                }
                lhs / rhs
            }
            _ => panic!("Invalid operator: {}", op),
        }
    };

    // Wrap and re-evaluate to minimize the result
    let temp_expr = Box::new(NumericLiteral { value: result });
    evaluate(temp_expr, scope)
}


pub fn static_type_check<'a>(value: Box<dyn RuntimeValue>, type_ideal: Attr, complex: Option<Identifier>, scope: &'a RefCell<Scope>) {
    match type_ideal {
        Attr::Numeric => {
            if is_numeric_val(&value) || value.as_any().downcast_ref::<NilVal>().is_some() {} 
            else {panic!("Incorrect Type Assignement");}
        },
        Attr::Bool => {
            if value.as_any().downcast_ref::<BooleanVal>().is_some() || value.as_any().downcast_ref::<NilVal>().is_some(){}
            else {panic!("Incorrect Type Assignement");}
        },
        Attr::Object => {
            if value.as_any().downcast_ref::<ObjectVal>().is_some() || value.as_any().downcast_ref::<NilVal>().is_some(){}
            else {panic!("Incorrect Type Assignement");}
        },
        Attr::ComplexKind => {
            let unwrap = complex.unwrap_or_else(|| panic!("Complex Struct defined without complex flag specification"));
            complex_static_type_check(unwrap, value, scope);
        }
        _ => {
            todo!()
        }
    }
}

macro_rules! is_numeric_val {
    ($value:expr, $( $t:ty ),*) => {
        $( $value.as_any().downcast_ref::<NumericVal<$t>>().is_some() )||*
    };
}

fn is_numeric_val(value: &Box<dyn RuntimeValue>) -> bool {
    is_numeric_val!(
        value,
        i8, i16, i32, i64,
        u8, u16, u32, u64,
        f32, f64
    )
}



fn complex_static_type_check<'a>(ideal: Identifier, value: Box<dyn RuntimeValue>, scope: &'a RefCell<Scope>) {
    let lookup = eval_identifier(&ideal, scope);
    if let RuntimeValueServe::Ref(lookup_unwrap) = lookup{
       if lookup_unwrap.as_any().downcast_ref::<ObjectVal>().is_some(){
            let lookup_refined = lookup_unwrap.as_any().downcast_ref::<ObjectVal>().unwrap();
            let v_refined = value.as_any().downcast_ref::<ObjectLiteralVal>().unwrap();
            for prop in lookup_refined.properties.iter() {
                let k = prop.0.as_str();
                let v = prop.1;
                if v_refined.properties.get(k).is_some(){}
                else {panic!("Incorrect Type Assignement");}

                if let Attr::Complex(ref cmplx) = v.clone() {
                    static_type_check(v_refined.properties.get(k).unwrap().clone(), Attr::ComplexKind, Some(Identifier{symbol: cmplx.clone()}), scope);
                }else{
                    static_type_check(v_refined.properties.get(k).unwrap().clone(), v.clone(), None, scope);
                }

                
            }
            for prop in v_refined.properties.iter() {
                if !lookup_refined.properties.get(prop.0).is_some() {
                    panic!("Extra Fields")
                }
            }

       }
    }else{
        panic!("I honestly dont get paid enough for this");
    }
}
