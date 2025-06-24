use std::collections::HashMap;

use crate::values::RuntimeValue;

#[derive(Clone)]
pub enum Parent{
    Scope(Box<Scope>),
    Nil,
}

#[derive(Clone)]
pub struct Scope{
    parent: Parent,
    variables: HashMap<String, Box<dyn RuntimeValue>>
}

impl Scope{
    pub fn new(parent_scope: Parent) -> Scope {
       Scope{
           parent: parent_scope,
           variables: HashMap::new(),
       }
    }
    pub fn var_decl(&mut self, varname: String, value: Box<dyn RuntimeValue>) ->  Box<dyn RuntimeValue> {
        if self.variables.get(&varname).is_some() {panic!("{}", format!("variable already defined [{:?}]", varname));}
        self.variables.insert(varname, value.clone());
        return value;
    }
    pub fn var_assign(&mut self, varname: String, value: Box<dyn RuntimeValue>) -> Box<dyn RuntimeValue> {
        let mut env = self.resolve_mut(&varname);
        let mut k = env.variables.get_mut(&varname).unwrap();
        *k = value.clone();
        return value;
    }
    pub fn resolve(&self, varname: &String) -> &Scope {
        if self.variables.get(varname).is_some() {
            return self; 
        }
        match &self.parent {
            Parent::Nil => {
                panic!("Variable doesn't exist in the current scope");
            },
            Parent::Scope(s) => {
                return s.resolve(varname);
            }
        }
    }
    pub fn resolve_mut(&mut self, varname: &String) -> &mut Scope {
        if self.variables.get(varname).is_some() {
            return self;
        }

        match &mut self.parent {
            Parent::Scope(s) => s.resolve_mut(varname),
            Parent::Nil => panic!("Variable doesn't exist in the current scope"),
        }
    }
    pub fn loopup(&self, varname: String) -> &Box<dyn RuntimeValue> {
        let env = self.resolve(&varname);
        return env.variables.get(&varname).unwrap();
    }
}
