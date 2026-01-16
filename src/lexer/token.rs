//! Token definitions for SpecterScript
//! Based on grammar.md specification

use std::fmt;

/// A span in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, length: usize, line: usize, column: usize) -> Self {
        Self { start, length, line, column }
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A token with its kind and location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: impl Into<String>) -> Self {
        Self { kind, span, lexeme: lexeme.into() }
    }
}

/// All possible token types in SpecterScript (based on grammar.md)
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Yes,        // true
    No,         // false
    Nil,        // null

    // Identifiers
    Identifier(String),

    // Variable/Constant keywords
    Fb,         // variable declaration
    Cn,         // constant declaration

    // Types
    Nb,         // number
    Int,        // integer
    Fl,         // float
    Wrd,        // string type
    By,         // byte
    Chr,        // char
    Any,        // any type
    Void,       // void type
    Lst,        // list type
    Map,        // map type
    Tup,        // tuple type
    Set,        // set type

    // Function keywords
    Fn,         // function
    Arrow,      // -> return
    FatArrow,   // => lambda

    // Control flow
    If,
    Elif,
    Else,
    Do,         // block start
    End,        // block end
    While,
    For,
    Each,       // for-each
    In,
    Break,
    Continue,
    Match,

    // Struct/Enum
    Struct,
    Enum,
    Trait,
    Type,       // type alias
    Mod,        // module
    Use,        // import
    Export,     // export
    As,         // alias

    // Error handling
    Try,
    Catch,
    Finally,
    Err,        // throw error
    Assert,

    // Memory
    Move,
    Unsafe,
    Inline,
    Free,

    // Async
    Async,
    Await,
    Spawn,

    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Caret,          // ^ (power)
    Ampersand,      // & (bitwise and / borrow)
    Pipe,           // | (bitwise or)
    CaretPipe,      // ^| (xor)
    Tilde,          // ~ (bitwise not)
    ShiftLeft,      // <<
    ShiftRight,     // >>
    Append,         // << (also used for append to list)

    // Comparison
    Equal,          // ==
    NotEqual,       // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    Bang,           // ! (logical not)

    // Assignment
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    StarAssign,     // *=
    SlashAssign,    // /=

    // Delimiters
    LeftParen,      // (
    RightParen,     // )
    LeftBracket,    // [
    RightBracket,   // ]
    LeftBrace,      // {
    RightBrace,     // }
    Comma,          // ,
    Colon,          // :
    Semicolon,      // ;
    Dot,            // .
    DotDot,         // .. (range inclusive)
    DotDotLess,     // ..< (range exclusive)
    Hash,           // # (length operator)
    Question,       // ? (optional type / ternary)

    // Channel operators
    LeftArrow,      // <- (receive)
    SendArrow,      // <- (send, context dependent)

    // Newline (significant in some contexts)
    Newline,

    // Special
    Eof,
    Error(String),
}

impl TokenKind {
    /// Try to convert an identifier to a keyword
    pub fn keyword_from_str(s: &str) -> Option<TokenKind> {
        match s {
            // Variable/constant
            "fb" => Some(TokenKind::Fb),
            "cn" => Some(TokenKind::Cn),
            
            // Types
            "nb" => Some(TokenKind::Nb),
            "int" => Some(TokenKind::Int),
            "fl" => Some(TokenKind::Fl),
            "wrd" => Some(TokenKind::Wrd),
            "by" => Some(TokenKind::By),
            "chr" => Some(TokenKind::Chr),
            "any" => Some(TokenKind::Any),
            "void" => Some(TokenKind::Void),
            "lst" => Some(TokenKind::Lst),
            "map" => Some(TokenKind::Map),
            "tup" => Some(TokenKind::Tup),
            "set" => Some(TokenKind::Set),
            
            // Booleans
            "yes" => Some(TokenKind::Yes),
            "no" => Some(TokenKind::No),
            "nil" => Some(TokenKind::Nil),
            
            // Functions
            "fn" => Some(TokenKind::Fn),
            
            // Control flow
            "if" => Some(TokenKind::If),
            "elif" => Some(TokenKind::Elif),
            "else" => Some(TokenKind::Else),
            "do" => Some(TokenKind::Do),
            "end" => Some(TokenKind::End),
            "while" => Some(TokenKind::While),
            "for" => Some(TokenKind::For),
            "each" => Some(TokenKind::Each),
            "in" => Some(TokenKind::In),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "match" => Some(TokenKind::Match),
            
            // Struct/Enum
            "struct" => Some(TokenKind::Struct),
            "enum" => Some(TokenKind::Enum),
            "trait" => Some(TokenKind::Trait),
            "type" => Some(TokenKind::Type),
            "mod" => Some(TokenKind::Mod),
            "use" => Some(TokenKind::Use),
            "export" => Some(TokenKind::Export),
            "as" => Some(TokenKind::As),
            
            // Error handling
            "try" => Some(TokenKind::Try),
            "catch" => Some(TokenKind::Catch),
            "finally" => Some(TokenKind::Finally),
            "err" => Some(TokenKind::Err),
            "assert" => Some(TokenKind::Assert),
            
            // Memory
            "move" => Some(TokenKind::Move),
            "unsafe" => Some(TokenKind::Unsafe),
            "inline" => Some(TokenKind::Inline),
            "free" => Some(TokenKind::Free),
            
            // Async
            "async" => Some(TokenKind::Async),
            "await" => Some(TokenKind::Await),
            "spawn" => Some(TokenKind::Spawn),
            
            _ => None,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Integer(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Identifier(s) => write!(f, "{}", s),
            TokenKind::Error(s) => write!(f, "ERROR: {}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}
