//! Lexer module for SpecterScript
//!
//! Converts source code into a stream of tokens, handling indentation-based blocks.

mod token;
mod scanner;

pub use token::{Token, TokenKind, Span};
pub use scanner::Lexer;
