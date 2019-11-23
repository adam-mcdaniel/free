use crate::{Error, Value, add_to_compiled};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    scope: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            scope: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: impl ToString, value: Value) -> Result<(), Error> {
        if let Ok(previous_value) = self.get(name.to_string()) {
            previous_value.free();
        }
        
        self.scope.insert(name.to_string(), value.copy()?);
        Ok(())
    }

    pub fn define_no_cp(&mut self, name: impl ToString, value: Value) {
        if let Ok(previous_value) = self.get(name.to_string()) {
            previous_value.free();
        }
        
        self.scope.insert(name.to_string(), value);
    }

    pub fn get(&mut self, name: impl ToString) -> Result<Value, Error> {
        match self.scope.get(&name.to_string()) {
            Some(val) => Ok(*val),
            None => Err(Error::VariableNotDefined(name.to_string(), self.clone())),
        }
    }

    pub fn free(&mut self) {
        for value in self.scope.values() {
            // value.free();
            if !value.is_ref() {
                value.free();
            } else {
                add_to_compiled(format!("NOT FREEING {:#?}", value));
            }
        }
    }
}
