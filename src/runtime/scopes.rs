use crate::interpreter::{unwrap_runtime_value_serve, RuntimeValueServe};
use crate::lexer::Flags;
use crate::values::{NativeFnValue, NilVal, RuntimeValue};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone)]
pub struct VariableEntry {
    pub value: Box<dyn RuntimeValue>,
    pub flags: Vec<Flags>,
    pub locked: bool,
}

#[derive(Clone)]
pub enum Parent{
    Scope(Box<Scope>),
    Nil,
}

#[derive(Clone)]
pub struct Scope{
    parent: Parent,
    variables: HashMap<String, VariableEntry>
}

impl Scope{
    pub fn new(parent_scope: Parent) -> Scope {
       Scope{
           parent: parent_scope,
           variables: HashMap::new(),
       }
    }
    pub fn var_decl(&mut self, varname: String, value: Box<dyn RuntimeValue>, flags: Vec<Flags>) ->  Box<dyn RuntimeValue> {
        if self.variables.get(&varname).is_some() {panic!("{}", format!("variable already defined [{:?}]", varname));}
        self.variables.insert(varname, VariableEntry{value: value.clone(), flags, locked: false});
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

    pub fn var_assign(&mut self, varname: String, value: Box<dyn RuntimeValue>) -> Box<dyn RuntimeValue> {
        if self.loopup_flags(varname.clone()).contains(&Flags::Const_f) {panic!("Cannot reassign variable marked with flag: <const>")}

        let env = self.resolve_mut(&varname);
        let k = env.variables.get_mut(&varname).unwrap();
        while k.locked{};
        k.locked = true;
        k.value = value.clone();
        k.locked = false;
        value
    }

    pub fn loopup(&self, varname: String) -> &Box<dyn RuntimeValue> {
        let env = self.resolve(&varname);
        &env.variables.get(&varname).unwrap().value
    }

    pub fn loopup_flags(&self, varname: String) -> &Vec<Flags> {
        let env = self.resolve(&varname);
        &env.variables.get(&varname).unwrap().flags
    }
}

pub fn init<'a>() -> Scope {
   let mut env = Scope::new(Parent::Nil);
   env.var_decl("log".to_string(), Box::new(NativeFnValue{call: Box::new(log_fn as fn(_, _) -> _)}), vec![Flags::Const_f]);

   env
}

fn log_fn<'a>(args: Vec<RuntimeValueServe>, _scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    println!("{}", unwrap_runtime_value_serve(args[0].clone()));
    return RuntimeValueServe::Owned(Box::new(NilVal{}))
}

// finish user def fns
// refine tests
// recheck makefile
// make repl
// check .io
// hc
