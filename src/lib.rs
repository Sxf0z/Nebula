//! SpecterScript - A high-performance scripting language
//! v0.9: Optimized bytecode VM with NaN-boxed values

pub mod error;
pub mod lexer;
pub mod parser;
pub mod interp;
pub mod builtins;
pub mod vm;
pub mod ext;

pub use error::{SpectreError, SpectreResult, ErrorCode};
pub use lexer::{Lexer, Token, TokenKind, Span};
pub use parser::{Parser, Program};
pub use interp::{Interpreter, Value, Environment};
pub use vm::{VM, Compiler, Chunk, OpCode};
pub use ext::{Extension, ExtFunction, ExtensionContext, ExtensionRegistry};
