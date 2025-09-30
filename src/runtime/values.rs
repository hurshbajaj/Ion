use ion_macros::RuntimeValue;
use num_traits::Num;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use crate::ast::Identifier;
use crate::interpreter::RuntimeValueServe;
use crate::lexer::Attr;
use crate::scopes::Scope;

#[derive(PartialEq)]
pub enum RuntimeValueType {
    Nil,

    Numeric,
    String,
    Boolean,

    ObjectVal,
    
    ObjectLiteralVal,
    ArrayVal,
    ArrayLiteralVal,
    
    FnStructVal,
    NativeFn,

    StmtExec
}

#[RuntimeValue(RuntimeValueType::StmtExec)]
pub struct StmtExecS{}

#[RuntimeValue(RuntimeValueType::Nil)]
pub struct NilVal{}

#[derive(Debug, Clone)]
pub struct NumericVal<T: Num + Debug = f64>{
    pub value: T,
}
impl<'a, T: Num + Debug + Display + Clone + 'static> RuntimeValue for NumericVal<T> {
    fn Type(&self) -> RuntimeValueType {
        RuntimeValueType::Numeric
    }

    fn clone_box(&self) -> Box<dyn RuntimeValue> {
        Box::new(Self {
            value: self.value.clone(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}


#[RuntimeValue(RuntimeValueType::Boolean)]
pub struct BooleanVal{
    pub val: bool,
}

#[RuntimeValue(RuntimeValueType::String)]
pub struct StrLiteral{
    pub content: String,
}

#[RuntimeValue(RuntimeValueType::ArrayVal)]
pub struct ArrayVal{
    pub attr: Attr,
    pub complex: Option<Identifier>,
    pub length: usize,
}

#[RuntimeValue(RuntimeValueType::ArrayVal)]
pub struct ArrayLiteralVal{
    pub entries: Vec<RuntimeValueServe>,
}

#[RuntimeValue(RuntimeValueType::ObjectVal)]
pub struct ObjectVal {
    pub properties: HashMap<String, Attr>,
}

pub struct ObjectLiteralVal {
    pub properties: HashMap<String, RuntimeValueServe>,
}

impl<'a> Debug for ObjectLiteralVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ObjectLiteralVal")
            .field("properties", &"Box<dyn RuntimeValue<'a>> map")
            .finish()
    }
}

impl Clone for ObjectLiteralVal {
    fn clone(&self) -> Self {
        Self {
            properties: self.properties
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }
}

impl RuntimeValue for ObjectLiteralVal {
    fn Type(&self) -> RuntimeValueType {
        RuntimeValueType::ObjectLiteralVal
    }

    fn clone_box(&self) -> Box<dyn RuntimeValue> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}


#[RuntimeValue(RuntimeValueType::FnStructVal)]
pub struct FuncStructVal{
    pub parameters: HashMap<String, Attr>,
    pub return_type: Attr,
}

pub trait Callable: Debug {
    fn call_fn(
        &self,
        args: Vec<RuntimeValueServe>,
        scope: &'static RefCell<Scope>,
    ) -> RuntimeValueServe;

    fn clone_box(&self) -> Box<dyn Callable>;
}

impl<'a> Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<'a, F> Callable for F
where
    F: Fn(Vec<RuntimeValueServe>, &'static RefCell<Scope>) -> RuntimeValueServe
        + Clone
        + Debug
        + 'static,
{
    fn call_fn(
        &self,
        args: Vec<RuntimeValueServe>,
        scope: &'static RefCell<Scope>,
    ) -> RuntimeValueServe {
        self(args, scope)
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }
}

#[RuntimeValue(RuntimeValueType::NativeFn)]
pub struct NativeFnValue{
    pub call: Box<dyn Callable>,
}

impl<'a> Clone for Box<dyn RuntimeValue> {
    fn clone(&self) -> Box<dyn RuntimeValue>{
        return self.clone_box()
    }
}

pub trait RuntimeValue: Display + Debug + Any{
    fn Type(&self) -> RuntimeValueType;
    fn clone_box(&self) -> Box<dyn RuntimeValue>;
    fn as_any(&self) -> &dyn Any;
}
