use std::fmt;
use crate::values::*;

pub use crate::runtime::complex_values_impls;

impl fmt::Display for NilVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nil")
    }
}

impl<T: num_traits::Num + fmt::Debug + fmt::Display> fmt::Display for NumericVal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for StrLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl fmt::Display for BooleanVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl fmt::Display for ObjectVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keys: Vec<String> = self.properties.keys().cloned().collect();
        write!(f, "Object({})", keys.join(", "))
    }
}

impl fmt::Display for ArrayVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.complex.clone() {
            None => write!(f, "Array(Type: {:?}, Length: {})", self.attr , self.length),
            Some(i) =>  write!(f, "Array(Type: Complex({}),  Length: {})", &(i.symbol), self.length)
        }
    }
}

impl fmt::Display for NativeFnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}

impl fmt::Display for StmtExecS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<stmt execution>")
    }
}
