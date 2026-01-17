use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::with_capacity(16), 
            parent: None,
        }
    }
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }
    pub fn parent(&self) -> Option<Rc<RefCell<Environment>>> {
        self.parent.clone()
    }
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            false
        }
    }
    pub fn is_defined_locally(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    pub fn locals(&self) -> &HashMap<String, Value> {
        &self.values
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
