use ion_macros::RuntimeValue;
use num_traits::Num;
use std::any::Any;
use std::fmt::Debug;

#[derive(PartialEq)]
pub enum RuntimeValueType {
    Nil,
    Numeric,
    Boolean,
    StmtExec,
}

#[RuntimeValue(RuntimeValueType::StmtExec)]
pub struct StmtExecS{}

#[RuntimeValue(RuntimeValueType::Nil)]
pub struct NilVal{}

#[RuntimeValue(RuntimeValueType::Numeric)]
pub struct NumericVal<T: Num + Debug = f64>{
    pub value: T,
}

#[RuntimeValue(RuntimeValueType::Boolean)]
pub struct BooleanVal{
    pub value: bool,
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
