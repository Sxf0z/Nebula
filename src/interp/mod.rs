//! Interpreter module for SpecterScript
//!
//! Tree-walking interpreter for AST evaluation.

mod value;
mod env;
mod eval;

pub use value::{Value, FunctionValue, LambdaValue, NativeFn};
pub use env::Environment;
pub use eval::Interpreter;
