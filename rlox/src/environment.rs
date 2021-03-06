use crate::error::*;
use crate::value::*;
use std::collections::HashMap;

pub struct Environment {
    pub values: HashMap<String, Value>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosed(parent: Box<Environment>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(parent),
        }
    }

    pub fn define(&mut self, name: &str, val: &Value) {
        self.values.insert(name.to_string(), val.clone());
    }

    pub fn assign(&mut self, name: &str, val: &Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
        return Err(RuntimeError::new(
            &format!("undefined variable '{}'", name),
            0));
        }

        self.values.insert(name.to_string(), val.clone());
        println!("{} {}", name, val);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        if let Some(val) = self.values.get(name) {
            return Ok(val.clone());
        }

        if let Some(enc) = &self.enclosing {
            return enc.get(name);
        }

        Err(RuntimeError::new(
            &format!("undefined variable '{}'", name),
            0,
        ))
    }
    //
}
