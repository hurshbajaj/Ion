use crate::interpreter::{unwrap_runtime_value_serve, RuntimeValueServe};
use crate::lexer::Flags;
use crate::values::{NativeFnValue, NilVal};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};

use super::values::StrLiteral;

#[derive(Clone)]
pub struct VariableEntry {
    pub value: RuntimeValueServe,
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
    pub parent: Parent,
    pub variables: HashMap<String, VariableEntry>
}

impl Scope{
    pub fn new(parent_scope: Parent) -> Scope {
       Scope{
           parent: parent_scope,
           variables: HashMap::new(),
       }
    }
    pub fn var_decl(&mut self, varname: String, value: RuntimeValueServe, flags: Vec<Flags>) ->  RuntimeValueServe {
        if self.variables.get(&varname).is_some() {panic!("{}", format!("variable already defined [{:?}]", varname));}
        match value.clone() {
            RuntimeValueServe::Owned(_) => {
                self.variables.insert(varname, VariableEntry{value: value.clone(), flags, locked: false});
            },
            RuntimeValueServe::Ref(_) => {
                self.variables.insert(varname, VariableEntry{value: value.clone(), flags, locked: false});
            }
        }
        return value;
    }

    pub fn resolve(&self, varname: &String) -> &Scope {
        if self.variables.recursive_lookup(varname).is_some() {
            return self; 
        }
        match &self.parent {
            Parent::Nil => {
                panic!("Variable [{}] doesn't exist in the current scope!", varname.clone());
            },
            Parent::Scope(s) => {
                return s.resolve(varname);
            }
        }
    }
    pub fn resolve_mut(&mut self, varname: &String) -> &mut Scope {
        if self.variables.recursive_lookup(varname).is_some() {
            return self;
        }

        match &mut self.parent {
            Parent::Scope(s) => s.resolve_mut(varname),
            Parent::Nil => panic!("Variable doesn't exist in the current scope"),
        }
    }

    pub fn var_assign(&mut self, varname: String, value: RuntimeValueServe) -> RuntimeValueServe {
        if self.lookup_flags(varname.clone()).contains(&Flags::Const_f) {panic!("Cannot reassign variable marked with flag: <const>")}

        let env = self.resolve_mut(&varname);
        let k = env.variables.get_mut(&varname).unwrap();
        while k.locked{};
        k.locked = true;
        k.value = value.clone();
        k.locked = false;
        value
    }

    pub fn lookup(&self, varname: String) -> RuntimeValueServe {
        let env = self.resolve(&varname);
        env.variables.get(&varname).unwrap().value.clone()
    }

    pub fn lookup_flags(&self, varname: String) -> &Vec<Flags> {
        let env = self.resolve(&varname);
        &env.variables.get(&varname).unwrap().flags
    }
}

trait RecursiveLookup {
    fn recursive_lookup(&self, key: &String) -> Option<RuntimeValueServe>;
}

impl RecursiveLookup for HashMap<String, VariableEntry> {
    fn recursive_lookup(&self, key: &String) -> Option< RuntimeValueServe > {
        let entry = self.get(key).unwrap();
        match entry.value.clone() {
            RuntimeValueServe::Owned(v) => {
                Some(RuntimeValueServe::Owned(v))
            },
            RuntimeValueServe::Ref(s) => {
                self.recursive_lookup(&String::from(s.symbol))
            }
        }
    }
}

impl Default for RuntimeValueServe {
    fn default() -> Self {
        RuntimeValueServe::Owned(Box::new(NilVal{}))
    }
}

//native

pub fn init<'a>() -> Scope {
   let mut env = Scope::new(Parent::Nil);
   env.var_decl("log".to_string(), RuntimeValueServe::Owned(  Box::new(NativeFnValue{call: Box::new(log_fn as fn(_, _) -> _)}) ), vec![Flags::Const_f]);
   env.var_decl("get".to_string(), RuntimeValueServe::Owned(  Box::new(NativeFnValue{call: Box::new(get_fn as fn(_, _) -> _)}) ), vec![Flags::Const_f]);

   env
}

fn log_fn<'a>(args: Vec<RuntimeValueServe>, scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    use std::io::Write;
    for arg in args{
        let value = unwrap_runtime_value_serve(arg.clone(), scope);
        let output = format!("{}", value);
        // Process escape sequences (single backslash from source)
        let processed = process_escape_sequences(&output);
        print!("{}", processed);
    }
    // Flush stdout to ensure output appears immediately
    io::stdout().flush().expect("Failed to flush stdout");
    return RuntimeValueServe::Owned(Box::new(NilVal{}))
}

fn process_escape_sequences(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    'n' => {
                        chars.next();
                        result.push('\n');
                    }
                    't' => {
                        chars.next();
                        result.push('\t');
                    }
                    'r' => {
                        chars.next();
                        result.push('\r');
                    }
                    '\\' => {
                        chars.next();
                        result.push('\\');
                    }
                    '0' => {
                        chars.next();
                        result.push('\0');
                    }
                    '"' => {
                        chars.next();
                        result.push('"');
                    }
                    '\'' => {
                        chars.next();
                        result.push('\'');
                    }
                    _ => {
                        // Unknown escape sequence, just push the backslash and continue
                        result.push(ch);
                    }
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn get_fn<'a>(args: Vec<RuntimeValueServe>, _scope: &'static RefCell<Scope>) -> RuntimeValueServe {
    use std::io::Write;
    if args.len() > 0{
        panic!("get_fn() doesn't take any arguments.");
    }
    
    // Flush stdout before reading to ensure any prompts are displayed
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to 'get' || read line");
    
    // Trim the newline character(s) from the input
    let trimmed_input = input.trim_end().to_string();
    
    return RuntimeValueServe::Owned(Box::new(StrLiteral{content: trimmed_input}))
}
