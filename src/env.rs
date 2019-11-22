use crate::Value;
use std::collections::HashMap;

pub struct Env {
    scope: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            scope: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: impl ToString, value: Value) {
        self.scope.insert(name.to_string(), value);
    }

    pub fn get(&mut self, name: impl ToString) -> Value {
        *self.scope.get(&name.to_string()).unwrap()
    }

    pub fn free(&mut self) {
        for value in self.scope.values() {
            if !value.is_ref() {
                // println!("scope dropping {:#?}", value);
                value.free();
            } else {
                println!("NOT FREEING {:#?}", value);
            }
        }
    }
}
