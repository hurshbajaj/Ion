use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use crate::ast::{self, ArrMemberExpr, Array, ArrayLiteral, BinExpr, CallExpr, FnStruct, Identifier, MemberExpr, NodeType, NumericLiteral, Object, ObjectLiteral, Program, Stmt, Str, VarAsg, VarDeclaration};
use crate::lexer::Attr;
use crate::scopes::Scope;
use crate::values::{BooleanVal, FuncStructVal, NativeFnValue, NilVal, NumericVal, ObjectLiteralVal, ObjectVal, RuntimeValue, RuntimeValueType, StmtExecS};

use super::values::{ArrayLiteralVal, ArrayVal, StrLiteral};

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

#[derive(Debug)]
pub enum RuntimeValueServe {
    Owned(Box<dyn RuntimeValue>),
    Ref(Identifier),
}

impl fmt::Display for RuntimeValueServe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValueServe::Owned(value) => {
                // Use the existing Display implementation of the boxed value
                write!(f, "{}", value)
            },
            RuntimeValueServe::Ref(identifier) => {
                // Print just the symbol field from Identifier
                write!(f, "{}", identifier.symbol)
            },
        }
    }
}

impl Clone for RuntimeValueServe {
    fn clone(&self) -> Self {
        match self {
            RuntimeValueServe::Owned(b) => RuntimeValueServe::Owned(b.clone()),
            RuntimeValueServe::Ref(r) => RuntimeValueServe::Ref(r.clone()), 
        }
    }
}


pub fn evaluate<'a>(astnode: Box<dyn Stmt>, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    match astnode.kind() {
        ast::NodeType::NumericLiteralNode => {
            let val = astnode
                .as_any()
                .downcast_ref::<NumericLiteral<f64>>() // Still parsing as f64
                .unwrap()
                .value;

            let minimized = minimize_numeric(val); 

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
            eval_identifier(astnode.as_any().downcast_ref::<Identifier>().unwrap(), scope)

        }
        ast::NodeType::String => {
            eval_string(astnode.as_any().downcast_ref::<Str>().unwrap(), scope)

        }
        ast::NodeType::Nil => RuntimeValueServe::Owned(Box::new(NilVal {})),
        ast::NodeType::Bool => RuntimeValueServe::Owned(Box::new(BooleanVal {
            val: astnode
                .as_any()
                .downcast_ref::<ast::Bool>()
                .unwrap()
                .value,
        })),
        ast::NodeType::BinOp => {
            eval_bin_expr(astnode.as_any().downcast_ref::<BinExpr>().unwrap(), scope)
        },
        ast::NodeType::Object => {
            eval_obj_expr(astnode.as_any().downcast_ref::<Object>().unwrap(), scope)
        },
        ast::NodeType::ObjectLiteral => {
            eval_obj_literal_expr(astnode.as_any().downcast_ref::<ObjectLiteral>().unwrap(), scope)
        },
        ast::NodeType::Array => {
            eval_array_expr(astnode.as_any().downcast_ref::<Array>().unwrap(), scope)
        },
        ast::NodeType::ArrayLiteral => {
            eval_array_literal_expr(astnode.as_any().downcast_ref::<ArrayLiteral>().unwrap(), scope)
        }
        ast::NodeType::FnStruct => {
            eval_fn_struct(astnode.as_any().downcast_ref::<FnStruct>().unwrap(), scope)
        },
        ast::NodeType::CallExpr => {
            eval_call_expr(astnode.as_any().downcast_ref::<CallExpr>().unwrap(), scope)
        }
        ast::NodeType::Program => eval_program(astnode, scope),
        ast::NodeType::VarDecl => {
            eval_var_decl(astnode.as_any().downcast_ref::<VarDeclaration>().unwrap(), scope)
        },
        ast::NodeType::VarAsg => {
            eval_var_asg(astnode.as_any().downcast_ref::<VarAsg>().unwrap(), scope)
        },
        ast::NodeType::MemberExpr => {
            eval_membr_expr(astnode.as_any().downcast_ref::<MemberExpr>().unwrap(), scope)
        },
        ast::NodeType::ArrMemberExpr => {
            eval_arr_membr_expr(astnode.as_any().downcast_ref::<ArrMemberExpr>().unwrap(), scope)
        },
        _ => panic!("Can't evaluate AST node yet: {:?}", astnode),
    }
}

fn eval_arr_membr_expr(unwrap: &ArrMemberExpr, scope: &'static RefCell<Scope>) -> RuntimeValueServe {    
    let arr_eval = evaluate(unwrap.arr.clone(), scope);
    let i_shell = evaluate(unwrap.clone().index, scope);
    
    let i = extract_as_i64(i_shell) as usize;

    match arr_eval {
        RuntimeValueServe::Owned(arr_val) => {
            let arr_val = unwrap_runtime_value_serve(RuntimeValueServe::Owned(arr_val), scope);
            let arr = arr_val.as_any().downcast_ref::<ArrayLiteralVal>().expect("Array indexing can only be executed on an array.");
            let i_val = arr.entries[i].clone();
            i_val
        },
        _ => {
            panic!("Cannot work with raw REF in runtime...");
        }   
    }
}

fn eval_string(unwrap: &Str, _scope: &RefCell<Scope>) -> RuntimeValueServe {
    RuntimeValueServe::Owned(Box::new(StrLiteral{content: unwrap.content.clone()}))
}

fn eval_array_literal_expr(unwrap: &ArrayLiteral, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    let mut arr = ArrayLiteralVal { entries: vec![] };
    for entry in &unwrap.entries {
        let val = evaluate(  entry.clone() , scope); 
        if let RuntimeValueServe::Owned(_) = val.clone(){
            arr.entries.push(val.clone());
        }else if let RuntimeValueServe::Ref(_) = val.clone(){
            arr.entries.push(val);
        }

    }
    RuntimeValueServe::Owned(Box::new(arr))
}

fn eval_array_expr(unwrap: &Array, scope: &RefCell<Scope>) -> RuntimeValueServe {
    let arr = match &unwrap.complex_attr {
        Some(str) => {
            ArrayVal { attr: unwrap.attr.clone(), complex: Some(Identifier{symbol: str.to_string()}), length: unwrap.length }
        },
        None => ArrayVal { attr: unwrap.attr.clone(), complex: None, length: unwrap.length }
    };
    RuntimeValueServe::Owned(Box::new(arr))
}

fn eval_membr_expr(unwrap: &MemberExpr, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    let obj_eval = evaluate(unwrap.obj.clone(), scope);
    let prop_name = &unwrap.prop.as_any().downcast_ref::<Identifier>().unwrap().symbol;

    match obj_eval {
        RuntimeValueServe::Owned(obj_val) => {
            let obj_val = unwrap_runtime_value_serve(RuntimeValueServe::Owned(obj_val), scope);
            let obj = obj_val.as_any().downcast_ref::<ObjectLiteralVal>().unwrap();
            let prop_val = obj.properties.get(prop_name)
                .unwrap_or_else(|| panic!("Property '{}' not found", prop_name))
                .clone();
            prop_val
        },
        _ => {
            panic!("Cannot work with raw REF in runtime...");
        }   
    }
}

fn eval_call_expr<'a>(unwrap: &CallExpr, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    let args = unwrap.args.iter().map(|a| evaluate(a.clone(), scope) ).collect();
    let func = unwrap_runtime_value_serve( evaluate(unwrap.call_to.clone() , scope), scope);

    if func.Type() == RuntimeValueType::NativeFn {
        let result = func.as_any().downcast_ref::<NativeFnValue>().unwrap().call.call_fn(args, scope);
        return result;
    }
    todo!();
}

//ptr restructure
fn eval_obj_literal_expr<'a>(unwrap: &ObjectLiteral, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    let mut object = ObjectLiteralVal { properties: HashMap::new() };
    for prop in &unwrap.properties {
        let ts =
            match prop.value.kind() {
                ast::NodeType::Identifier => {
                    RuntimeValueServe::Ref(prop.value.as_any().downcast_ref::<Identifier>().unwrap().clone())
                },
                _ =>  evaluate(prop.value.clone(), scope)
            };

            object.properties.insert(prop.key.clone(), ts);
    }
    RuntimeValueServe::Owned(Box::new(object))
}

fn eval_obj_expr<'a>(unwrap: &Object, _scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    let mut object = ObjectVal { properties: HashMap::new() };
    for prop in &unwrap.properties {
        let val = prop.value.clone(); 

        object.properties.insert(prop.key.clone(), val);
    }
    RuntimeValueServe::Owned(Box::new(object))
}

fn eval_fn_struct<'a>(unwrap: &FnStruct, _scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    let mut fn_struct = FuncStructVal{parameters: HashMap::new(), return_type: (&unwrap.ret_type).clone()};
    for param in &unwrap.params {
        let val = param.param_type.clone();

        fn_struct.parameters.insert(param.param.clone(), val);
    }

    RuntimeValueServe::Owned(Box::new(fn_struct))
}

pub fn eval_var_asg<'a>(unwrap: &VarAsg, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    match unwrap.lhs.kind() {
        NodeType::Identifier => {
            var_asg_ident(unwrap, scope)
        },
        NodeType::MemberExpr => {
            var_asg_membr_expr(unwrap, scope)
        }
        _ => {
            panic!("Can't assign value to the same.");
        }
    }
}

pub fn var_asg_ident(unwrap: &VarAsg, scope: &'static RefCell<Scope>) -> RuntimeValueServe{
    let lhs_refined = unwrap.lhs.as_any().downcast_ref::<Identifier>().unwrap(); 
    
    // Evaluate once and reuse
    let evaluated = evaluate(unwrap.rhs.clone(), scope);
    
    let ts = match unwrap.rhs.clone().kind() {
        ast::NodeType::Identifier => {
            RuntimeValueServe::Ref(unwrap.rhs.clone().as_any().downcast_ref::<Identifier>().unwrap().clone())
        },
        _ => evaluated.clone()
    };
    
    let refined_rhs = unwrap_runtime_value_serve(evaluated.clone(), scope);

    let scope_refined = scope.borrow().clone();
    let _ = scope_refined.resolve(&lhs_refined.symbol);

    let f_flag = scope_refined.lookup_flags(lhs_refined.symbol.clone()).iter().find_map(|token_type| {
        if let crate::lexer::Flags::Struct_f(attr) = token_type {
            Some(attr.clone())
        } else {
            None
        }
    }).unwrap_or_else(|| panic!("Missing flag <structure> not found in Associated Variable Flags"));

    let complex_t: Option<Identifier> = scope_refined.lookup_flags(lhs_refined.symbol.clone()).iter().find_map(|token_type| {
        if let crate::lexer::Flags::Complex_f(Attr::Complex(attr)) = token_type {
            Some(Identifier { symbol: attr.clone() })
        } else {
            None
        }
    });

    static_type_check(refined_rhs.clone(), f_flag, complex_t, scope);

    scope
        .borrow_mut()
        .var_assign(lhs_refined.symbol.clone(), ts);

    RuntimeValueServe::Owned(Box::new(StmtExecS {}))
}

pub fn var_asg_membr_expr(unwrap: &VarAsg, scope: &'static RefCell<Scope>) -> RuntimeValueServe{
    let lhs_refined = unwrap.lhs.as_any().downcast_ref::<MemberExpr>().unwrap(); 
    let obj_as_ident = lhs_refined.obj.as_any().downcast_ref::<Identifier>().unwrap().symbol.clone();

    let new_obj_shell = scope.borrow().clone().lookup(obj_as_ident.clone());
    let new_obj_weak = match new_obj_shell {
        RuntimeValueServe::Owned(v) => v,
        _ => {panic!()}
    };
    let mut new_obj= new_obj_weak.clone().as_any().downcast_ref::<ObjectLiteralVal>().unwrap().clone();
    let prop_mut = new_obj.properties.get_mut(&(lhs_refined.prop.as_any().downcast_ref::<Identifier>().unwrap().symbol.clone())).expect("Property doesn't exist.");

    match unwrap.rhs.clone().kind() {
        ast::NodeType::Identifier => {
            *prop_mut = RuntimeValueServe::Ref(unwrap.rhs.clone().as_any().downcast_ref::<Identifier>().unwrap().clone());
        },
       _ =>  {
           *prop_mut = evaluate(unwrap.rhs.clone(), scope);
       }
    }

    let scope_refined = scope.borrow().clone();
    let _ = scope_refined.resolve(&obj_as_ident);

    let complex_t: Option<Identifier> = scope_refined.lookup_flags(obj_as_ident.clone()).iter().find_map(|token_type| {
        if let crate::lexer::Flags::Complex_f(Attr::Complex(attr)) = token_type {
            Some(Identifier { symbol: attr.clone() })
        } else {
            None
        }
    });

    static_type_check(Box::new(new_obj.clone()), Attr::ComplexKind, complex_t, scope);

    scope
        .borrow_mut()
        .var_assign(obj_as_ident.clone(), RuntimeValueServe::Owned(Box::new(new_obj.clone())));

    RuntimeValueServe::Owned(Box::new(StmtExecS {}))
}

pub fn eval_var_decl<'a>(unwrap: &VarDeclaration, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    if unwrap.identifier == "_"{
        panic!("Token (_) cannot be used as an identifier.");
    }
    
    // Evaluate once and reuse
    let evaluated = evaluate(unwrap.value.clone(), scope);
    
    let ts = match unwrap.value.clone().kind() {
        ast::NodeType::Identifier => {
            RuntimeValueServe::Ref(unwrap.value.clone().as_any().downcast_ref::<Identifier>().unwrap().clone())
        },
        _ => evaluated.clone()
    };
    
    let val_to_store = unwrap_runtime_value_serve(evaluated.clone(), scope);

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
        .var_decl(unwrap.identifier.clone(), ts, unwrap.flags.clone());

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

fn eval_identifier<'a>( unwrap: &Identifier, scope: &'static RefCell<Scope> ) -> RuntimeValueServe {
    scope.borrow().clone().lookup(unwrap.clone().symbol.to_string())
}


fn eval_program<'a>(astnode: Box<dyn Stmt>, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    let program = astnode.as_any().downcast_ref::<Program>().unwrap();

    if program.body.is_empty() {
        return RuntimeValueServe::Owned(Box::new(NilVal {}));
    }

    for stmt in &program.body[..program.body.len() - 1] {
        let _ = evaluate(stmt.clone(), scope);
    }

    evaluate(program.body.last().unwrap().clone(), scope)
}

fn eval_bin_expr<'a>(unwrap: &BinExpr, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
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
            } else if lhs_val.Type() == RuntimeValueType::String && rhs_val.Type() == RuntimeValueType::String{
                return RuntimeValueServe::Owned(Box::new(StrLiteral{content: lhs_val.as_any().downcast_ref::<StrLiteral>().unwrap().content.clone() + rhs_val.as_any().downcast_ref::<StrLiteral>().unwrap().content.clone().as_str()}))
            }
            
        }
        _ => {
            panic!("Cannot operate using raw REF during runtime...");
        }
    }

    RuntimeValueServe::Owned(Box::new(NilVal {}))
}

fn extract_as_i64(val: RuntimeValueServe) -> i64 {
    match val {
        RuntimeValueServe::Owned(v) => {
            let any = v.as_any();
            extract_numeric!(any, i64, [u8, u16, u32, u64, i8, i16, i32, i64]);
            panic!("Expected integer-compatible value for op");
        },
        RuntimeValueServe::Ref(v) => {
            let any = v.as_any();
            extract_numeric!(any, i64, [u8, u16, u32, u64, i8, i16, i32, i64]);
            panic!("Expected integer-compatible value for op");
        }
    }
}
fn extract_as_f64(val: RuntimeValueServe) -> f64 {
    match val {
        RuntimeValueServe::Owned(v) => {
            let any = v.as_any();
            extract_numeric!(any, f64, [f64, f32, u8, u16, u32, u64, i8, i16, i32, i64]);
            panic!("Expected numeric value for f64 cast");
        },
        RuntimeValueServe::Ref(v) => {
            let any = v.as_any();
            extract_numeric!(any, f64, [f64, f32, u8, u16, u32, u64, i8, i16, i32, i64]);
            panic!("Expected numeric value for f64 cast");
        }

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
    lhs_val: RuntimeValueServe,
    rhs_val: RuntimeValueServe,
    op: &str,
    scope: &'static RefCell<Scope>,
) -> RuntimeValueServe {

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
            },
            "%" => {
                if rhs == 0.0 {
                    panic!("Modulo by zero");
                }
                lhs % rhs
            },
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
            },
            "%" => {
                if rhs == 0.0 {
                    panic!("Modulo by zero");
                }
                lhs % rhs
            },
            _ => panic!("Invalid operator: {}", op),
        }
    };

    let temp_expr = Box::new(NumericLiteral { value: result });
    evaluate(temp_expr, scope)
}


pub fn static_type_check<'a>(value: Box<dyn RuntimeValue>, type_ideal: Attr, complex: Option<Identifier>, scope: &'static RefCell<Scope>) {
    match type_ideal {
        Attr::Numeric => {
            if is_numeric_val(&value) || value.as_any().downcast_ref::<NilVal>().is_some() {} 
            else {panic!("Incorrect Type Assignement");}
        },
        Attr::String => {
            if value.as_any().downcast_ref::<StrLiteral>().is_some() || value.as_any().downcast_ref::<NilVal>().is_some() {} 
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
        Attr::Array => {
            if value.as_any().downcast_ref::<ArrayVal>().is_some() || value.as_any().downcast_ref::<NilVal>().is_some(){}
            else {panic!("Incorrect Type Assignement");}
        },
        Attr::ComplexKind => {
            let unwrap = complex.unwrap_or_else(|| panic!("Complex Struct defined without complex flag specification"));
            complex_static_type_check(unwrap, value, scope);
        }
        _ => {
            panic!("Can't use as type");
        }
    }
}

//openfull
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

fn complex_static_type_check<'a>(ideal: Identifier, value: Box<dyn RuntimeValue>, scope: &'static RefCell<Scope>) {
    if ideal.symbol == "anonymous" {
        return;
    }
    let lookup = eval_identifier(&ideal, scope);
    if let RuntimeValueServe::Owned(lookup_unwrap) = lookup{
       if lookup_unwrap.as_any().downcast_ref::<ObjectVal>().is_some(){
            let lookup_refined = lookup_unwrap.as_any().downcast_ref::<ObjectVal>().unwrap();
            let v_refined = value.as_any().downcast_ref::<ObjectLiteralVal>().unwrap_or_else(||{
                        panic!("{}", format!("Expected an object of type: {} | Found: {}", lookup_refined, value));
                });
            for prop in lookup_refined.properties.iter() {
                let k = prop.0.as_str();
                let v = prop.1;
                if v_refined.properties.get(k).is_some(){}
                else {panic!("Incorrect Type Assignement");}

                if let Attr::Complex(ref cmplx) = v.clone() {
                    static_type_check(unwrap_runtime_value_serve( v_refined.properties.get(k).unwrap_or_else(|| {
                        panic!("{}", format!("Property {:?} doesn't exist on expression {:?}", k, v_refined));
                    }).clone(), scope ) , Attr::ComplexKind, Some(Identifier{symbol: cmplx.clone()}), scope);
                }else{
                    static_type_check(unwrap_runtime_value_serve(v_refined.properties.get(k).unwrap_or_else(||{
                        panic!("{}", format!("Property {k} doesn't exist on expression {v_refined}"));
                    }).clone(), scope), v.clone(), None, scope);
                }

                
            }
            for prop in v_refined.properties.iter() {
                if !lookup_refined.properties.get(prop.0).is_some() {
                    panic!("Extra Fields")
                }
            }
       }
       if lookup_unwrap.as_any().downcast_ref::<ArrayVal>().is_some(){
            let lookup_refined = lookup_unwrap.as_any().downcast_ref::<ArrayVal>().unwrap();
            let v_refined = value.as_any().downcast_ref::<ArrayLiteralVal>().unwrap();
            if v_refined.entries.len() != lookup_refined.length {
                panic!("The size of an array must be strictly equal to that of its complex.");
            }
            for entry in v_refined.entries.clone() {
                static_type_check(unwrap_runtime_value_serve(entry.clone(), scope), lookup_refined.attr.clone(), lookup_refined.complex.clone(), scope);
            }
       }
    }else{
        panic!("I honestly dont get paid enough for this");
    }
}

pub fn unwrap_runtime_value_serve<'a>(value: RuntimeValueServe, scope: &'static RefCell<Scope>) -> Box<dyn RuntimeValue> {
    match value {
        RuntimeValueServe::Owned(val) => val,
        RuntimeValueServe::Ref(val) => {
                match scope.borrow().clone().lookup(val.clone().symbol.to_string()){
                    RuntimeValueServe::Owned(v) => v,
                    _ => {
                        panic!("Internal Interpreter Error; Unable to handle raw REF during runtime...");
                    }
                }
        },
    }
}

//openfull
