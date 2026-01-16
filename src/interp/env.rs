//! Runtime environment for variable bindings
//!
//! ## Performance Notes
//! - `get()` is O(1) per scope, O(n) in scope chain depth
//! - `define()` is O(1) amortized
//! - Values are cloned on read to avoid borrow conflicts
//! - Future: consider interning common values

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::value::Value;

/// Runtime environment with lexical scoping
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::with_capacity(16), // Pre-allocate for typical scope
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Get the parent environment
    pub fn parent(&self) -> Option<Rc<RefCell<Environment>>> {
        self.parent.clone()
    }

    /// Define a new variable in the current scope
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    /// Look up a variable in this scope or parent scopes
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    /// Assign to an existing variable (walks up scope chain)
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

    /// Check if a variable is defined locally
    pub fn is_defined_locally(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Get all local bindings
    pub fn locals(&self) -> &HashMap<String, Value> {
        &self.values
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
