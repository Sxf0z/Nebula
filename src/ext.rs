//! Native Extension API for SpecterScript
//!
//! Rust-first extension system with clear ownership rules.
//! 
//! # Design Principles
//! - No shared mutable state across boundary
//! - Zero-cost calls where possible
//! - Clear ownership: extensions borrow, runtime owns
//! - No unsafe required for extension authors
//!
//! # Usage
//! ```rust
//! use specterscript::ext::{Extension, ExtensionContext, ExtResult};
//!
//! struct MyExtension;
//!
//! impl Extension for MyExtension {
//!     fn name(&self) -> &str { "my_ext" }
//!     
//!     fn functions(&self) -> Vec<ExtFunction> {
//!         vec![ext_fn!("greet", greet)]
//!     }
//! }
//!
//! fn greet(ctx: &ExtensionContext, args: &[Value]) -> ExtResult<Value> {
//!     let name = args.get(0).and_then(|v| v.as_string()).unwrap_or("world");
//!     Ok(Value::String(format!("Hello, {}!", name)))
//! }
//! ```

use crate::interp::Value;
use crate::error::{SpectreError, SpectreResult, ErrorCode};

/// Result type for extension functions
pub type ExtResult<T> = Result<T, ExtError>;

/// Extension-specific error
#[derive(Debug, Clone)]
pub struct ExtError {
    pub message: String,
}

impl ExtError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { message: msg.into() }
    }
}

impl From<ExtError> for SpectreError {
    fn from(e: ExtError) -> Self {
        SpectreError::coded(ErrorCode::E080, e.message)
    }
}

/// Context passed to extension functions
/// 
/// Provides read-only access to runtime state.
/// Extensions cannot modify interpreter/VM state directly.
pub struct ExtensionContext<'a> {
    /// Function name being called
    pub fn_name: &'a str,
    /// Number of arguments passed
    pub argc: usize,
}

impl<'a> ExtensionContext<'a> {
    pub fn new(fn_name: &'a str, argc: usize) -> Self {
        Self { fn_name, argc }
    }
}

/// Native function signature
/// 
/// Takes context + arguments, returns a Value or error.
pub type NativeFn = fn(&ExtensionContext, &[Value]) -> ExtResult<Value>;

/// Describes a function exported by an extension
#[derive(Clone)]
pub struct ExtFunction {
    /// Function name (how it's called from SpecterScript)
    pub name: String,
    /// Minimum required arguments
    pub min_args: usize,
    /// Maximum arguments (None = unlimited)
    pub max_args: Option<usize>,
    /// The actual function pointer
    pub func: NativeFn,
}

impl ExtFunction {
    /// Create a simple function with variable args
    pub fn new(name: impl Into<String>, func: NativeFn) -> Self {
        Self {
            name: name.into(),
            min_args: 0,
            max_args: None,
            func,
        }
    }

    /// Create a function requiring exactly N args
    pub fn with_arity(name: impl Into<String>, arity: usize, func: NativeFn) -> Self {
        Self {
            name: name.into(),
            min_args: arity,
            max_args: Some(arity),
            func,
        }
    }

    /// Validate argument count
    pub fn validate_args(&self, argc: usize) -> ExtResult<()> {
        if argc < self.min_args {
            return Err(ExtError::new(format!(
                "{}: expected at least {} args, got {}",
                self.name, self.min_args, argc
            )));
        }
        if let Some(max) = self.max_args {
            if argc > max {
                return Err(ExtError::new(format!(
                    "{}: expected at most {} args, got {}",
                    self.name, max, argc
                )));
            }
        }
        Ok(())
    }
}

/// Trait for defining extensions
/// 
/// Implement this to create a native extension.
pub trait Extension: Send + Sync {
    /// Extension name (for debugging/logging)
    fn name(&self) -> &str;
    
    /// List of functions provided by this extension
    fn functions(&self) -> Vec<ExtFunction>;
    
    /// Called when extension is loaded (optional)
    fn on_load(&self) -> ExtResult<()> {
        Ok(())
    }
    
    /// Called when extension is unloaded (optional)
    fn on_unload(&self) -> ExtResult<()> {
        Ok(())
    }
}

/// Registry for loaded extensions
pub struct ExtensionRegistry {
    extensions: Vec<Box<dyn Extension>>,
    functions: std::collections::HashMap<String, ExtFunction>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
            functions: std::collections::HashMap::new(),
        }
    }

    /// Register an extension
    pub fn register(&mut self, ext: Box<dyn Extension>) -> ExtResult<()> {
        ext.on_load()?;
        
        for func in ext.functions() {
            self.functions.insert(func.name.clone(), func);
        }
        
        self.extensions.push(ext);
        Ok(())
    }

    /// Look up a function by name
    pub fn get_function(&self, name: &str) -> Option<&ExtFunction> {
        self.functions.get(name)
    }

    /// Call a registered function
    pub fn call(&self, name: &str, args: &[Value]) -> SpectreResult<Value> {
        let func = self.functions.get(name)
            .ok_or_else(|| SpectreError::coded(ErrorCode::E010, name))?;
        
        func.validate_args(args.len())?;
        
        let ctx = ExtensionContext::new(name, args.len());
        (func.func)(&ctx, args).map_err(|e| e.into())
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_add(_ctx: &ExtensionContext, args: &[Value]) -> ExtResult<Value> {
        let a = args.get(0).and_then(|v| v.as_number()).unwrap_or(0.0);
        let b = args.get(1).and_then(|v| v.as_number()).unwrap_or(0.0);
        Ok(Value::Number(a + b))
    }

    #[test]
    fn test_ext_function() {
        let func = ExtFunction::with_arity("add", 2, test_add);
        assert!(func.validate_args(2).is_ok());
        assert!(func.validate_args(1).is_err());
    }
}
