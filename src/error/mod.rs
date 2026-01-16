//! Error types for SpecterScript
//! 
//! Unified error system with memorable, dry error messages.

use thiserror::Error;
use crate::lexer::Span;

pub type SpectreResult<T> = Result<T, SpectreError>;

/// Error codes for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Parse errors (E001-E009)
    E001, // unexpected token
    E002, // expected identifier
    E003, // unclosed block
    E004, // invalid expression
    
    // Runtime errors (E010-E019)
    E010, // variable not found
    E011, // not callable
    E012, // wrong arg count
    E013, // nil access
    
    // Index errors (E020-E029)
    E020, // out of bounds
    E021, // invalid index type
    
    // Type errors (E030-E039)
    E030, // type mismatch
    E031, // not a number
    E032, // not iterable
    
    // Math errors (E040-E049)
    E040, // divide by zero
    
    // Recursion errors (E050-E059)
    E050, // stack overflow
    
    // IO errors (E060-E069)
    E060, // file not found
    E061, // io failed
    
    // Limit errors (E070-E079)
    E070, // execution timeout
    E071, // iteration limit
    
    // Extension errors (E080-E089)
    E080, // extension error
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::E001 => "E001",
            ErrorCode::E002 => "E002",
            ErrorCode::E003 => "E003",
            ErrorCode::E004 => "E004",
            ErrorCode::E010 => "E010",
            ErrorCode::E011 => "E011",
            ErrorCode::E012 => "E012",
            ErrorCode::E013 => "E013",
            ErrorCode::E020 => "E020",
            ErrorCode::E021 => "E021",
            ErrorCode::E030 => "E030",
            ErrorCode::E031 => "E031",
            ErrorCode::E032 => "E032",
            ErrorCode::E040 => "E040",
            ErrorCode::E050 => "E050",
            ErrorCode::E060 => "E060",
            ErrorCode::E061 => "E061",
            ErrorCode::E070 => "E070",
            ErrorCode::E071 => "E071",
            ErrorCode::E080 => "E080",
        }
    }
    
    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::E001 => "unexpected token",
            ErrorCode::E002 => "expected identifier",
            ErrorCode::E003 => "unclosed block",
            ErrorCode::E004 => "invalid expression",
            ErrorCode::E010 => "variable not found",
            ErrorCode::E011 => "not callable",
            ErrorCode::E012 => "wrong arg count",
            ErrorCode::E013 => "nil access",
            ErrorCode::E020 => "out of bounds",
            ErrorCode::E021 => "invalid index type",
            ErrorCode::E030 => "type mismatch",
            ErrorCode::E031 => "not a number",
            ErrorCode::E032 => "not iterable",
            ErrorCode::E040 => "divide by zero",
            ErrorCode::E050 => "stack overflow",
            ErrorCode::E060 => "file not found",
            ErrorCode::E061 => "io failed",
            ErrorCode::E070 => "execution timeout",
            ErrorCode::E071 => "iteration limit",
            ErrorCode::E080 => "extension error",
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum SpectreError {
    #[error("[{code}] {msg}")]
    Coded { 
        code: ErrorCode,
        msg: String,
        span: Option<Span>,
    },
    
    // Legacy variants for backward compatibility
    #[error("Lexer error at {span}: {message}")]
    Lexer { message: String, span: Span },

    #[error("Parse error at {span}: {message}")]
    Parse { message: String, span: Span },

    #[error("Type error at {span}: {message}")]
    Type { message: String, span: Span },

    #[error("Runtime error: {message}")]
    Runtime { message: String },

    #[error("Undefined variable: {name}")]
    UndefinedVariable { name: String },

    #[error("Index out of bounds: {index} (length: {length})")]
    IndexOutOfBounds { index: i64, length: usize },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },

    #[error("IO error: {message}")]
    Io { message: String },
}

impl SpectreError {
    // New constructor for coded errors
    pub fn coded(code: ErrorCode, detail: impl Into<String>) -> Self {
        let detail = detail.into();
        let msg = if detail.is_empty() {
            code.message().to_string()
        } else {
            format!("{}: {}", code.message(), detail)
        };
        SpectreError::Coded { code, msg, span: None }
    }
    
    pub fn coded_at(code: ErrorCode, detail: impl Into<String>, span: Span) -> Self {
        let detail = detail.into();
        let msg = if detail.is_empty() {
            code.message().to_string()
        } else {
            format!("{}: {}", code.message(), detail)
        };
        SpectreError::Coded { code, msg, span: Some(span) }
    }
    
    pub fn span(&self) -> Option<&Span> {
        match self {
            SpectreError::Coded { span, .. } => span.as_ref(),
            SpectreError::Lexer { span, .. } => Some(span),
            SpectreError::Parse { span, .. } => Some(span),
            SpectreError::Type { span, .. } => Some(span),
            _ => None,
        }
    }

    pub fn message(&self) -> String {
        match self {
            SpectreError::Coded { msg, .. } => msg.clone(),
            SpectreError::Lexer { message, .. } => message.clone(),
            SpectreError::Parse { message, .. } => message.clone(),
            SpectreError::Type { message, .. } => message.clone(),
            SpectreError::Runtime { message } => message.clone(),
            SpectreError::UndefinedVariable { name } => format!("variable not found: {}", name),
            SpectreError::IndexOutOfBounds { index, length } => 
                format!("out of bounds: {} (len {})", index, length),
            SpectreError::DivisionByZero => "divide by zero".to_string(),
            SpectreError::InvalidOperation { message } => message.clone(),
            SpectreError::Io { message } => message.clone(),
        }
    }
    
    pub fn code(&self) -> Option<ErrorCode> {
        match self {
            SpectreError::Coded { code, .. } => Some(*code),
            SpectreError::UndefinedVariable { .. } => Some(ErrorCode::E010),
            SpectreError::IndexOutOfBounds { .. } => Some(ErrorCode::E020),
            SpectreError::DivisionByZero => Some(ErrorCode::E040),
            _ => None,
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Special control flow - not really an error
#[derive(Debug, Clone)]
pub enum ControlFlow<V> {
    Return(V),
    Break,
    Continue,
}
