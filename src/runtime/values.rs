use ion_macros::RuntimeValue;
use num_traits::Num;
use std::fmt::Debug;

#[derive(PartialEq)]
pub enum RuntimeValueType {
    Nil,
    Numeric,
}

#[RuntimeValue(RuntimeValueType::Nil)]
pub struct NilVal{}

#[RuntimeValue(RuntimeValueType::Numeric)]
pub struct NumericVal<T: Num + Debug = f64>{
    pub value: T,
}

pub trait RuntimeValue: Debug{
    fn Type(&self) -> RuntimeValueType;
}
