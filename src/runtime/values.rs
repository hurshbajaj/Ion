use ion_macros::RuntimeValue;
use num_traits::Num;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::ast::Expr;
use crate::interpreter::RuntimeValueServe;
use crate::lexer::Attr;

#[derive(PartialEq)]
pub enum RuntimeValueType {
    Nil,
    Numeric,
    Boolean,
    ObjectVal,
    ObjectLiteralVal,
    StmtExec,
}

#[RuntimeValue(RuntimeValueType::StmtExec)]
pub struct StmtExecS{}

#[RuntimeValue(RuntimeValueType::Nil)]
pub struct NilVal{}

#[derive(Debug, Clone)]
pub struct NumericVal<T: Num + Debug = f64>{
    pub value: T,
}
impl<T: Num + Debug + Clone + 'static> RuntimeValue for NumericVal<T> {
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
#[RuntimeValue(RuntimeValueType::ObjectVal)]
pub struct ObjectVal {
    pub properties: HashMap<String, Attr>,
}

#[RuntimeValue(RuntimeValueType::ObjectLiteralVal)]
pub struct ObjectLiteralVal {
    pub properties: HashMap<String, Box<dyn RuntimeValue>>,
}

impl Clone for Box<dyn RuntimeValue> {
    fn clone(&self) -> Box<dyn RuntimeValue>{
        return self.clone_box()
    }
}

pub trait RuntimeValue: Debug + Any{
    fn Type(&self) -> RuntimeValueType;
    fn clone_box(&self) -> Box<dyn RuntimeValue>;
    fn as_any(&self) -> &dyn Any;
}
