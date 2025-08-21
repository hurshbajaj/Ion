use std::fmt;
use crate::values::*;
use crate::values_impls;

impl fmt::Display for ObjectLiteralVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (i, (k, v)) in self.properties.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, " }}")
    }
}

impl fmt::Display for ArrayLiteralVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (i, v) in self.entries.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", v)?;
        }
        write!(f, " }}")
    }
}

impl fmt::Display for FuncStructVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.parameters.keys().cloned().collect();
        write!(f, "fn({}) -> {:?}", params.join(", "), self.return_type)
    }
}
