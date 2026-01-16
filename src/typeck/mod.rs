//! Type checker module for SpecterScript
//!
//! Performs type inference and checking.

mod types;
mod infer;
mod check;

pub use types::*;
pub use check::TypeChecker;
