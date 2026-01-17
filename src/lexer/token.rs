use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
}
impl Span {
    pub fn new(start: usize, length: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            length,
            line,
            column,
        }
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
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}
impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            lexeme: lexeme.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Integer(i64),
    Float(f64),
    String(String),
    On,
    Off,
    Empty,
    Identifier(String),
    Perm,
    Give,
    Nb,
    Int,
    Fl,
    Wrd,
    By,
    Chr,
    Any,
    Void,
    Lst,
    Map,
    Tup,
    Set,
    Function,
    Arrow,
    FatArrow,
    If,
    Elsif,
    Else,
    Do,
    End,
    While,
    For,
    Each,
    In,
    Break,
    Continue,
    Match,
    Struct,
    Enum,
    Trait,
    Type,
    Mod,
    Use,
    Export,
    As,
    Try,
    Catch,
    Finally,
    Err,
    Assert,
    Move,
    Unsafe,
    Inline,
    Free,
    Async,
    Await,
    Spawn,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    CaretPipe,
    Tilde,
    ShiftLeft,
    ShiftRight,
    Append,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Bang,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Semicolon,
    Dot,
    DotDot,
    DotDotLess,
    Hash,
    Question,
    LeftArrow,
    SendArrow,
    Newline,
    Eof,
    Error(String),
}
impl TokenKind {
    pub fn keyword_from_str(s: &str) -> Option<TokenKind> {
        match s {
            "perm" => Some(TokenKind::Perm),
            "give" => Some(TokenKind::Give),
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
            "on" => Some(TokenKind::On),
            "off" => Some(TokenKind::Off),
            "empty" => Some(TokenKind::Empty),
            "fn" | "function" => Some(TokenKind::Function),
            "if" => Some(TokenKind::If),
            "elsif" => Some(TokenKind::Elsif),
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
            "struct" => Some(TokenKind::Struct),
            "enum" => Some(TokenKind::Enum),
            "trait" => Some(TokenKind::Trait),
            "type" => Some(TokenKind::Type),
            "mod" => Some(TokenKind::Mod),
            "use" => Some(TokenKind::Use),
            "export" => Some(TokenKind::Export),
            "as" => Some(TokenKind::As),
            "try" => Some(TokenKind::Try),
            "catch" => Some(TokenKind::Catch),
            "finally" => Some(TokenKind::Finally),
            "err" => Some(TokenKind::Err),
            "assert" => Some(TokenKind::Assert),
            "move" => Some(TokenKind::Move),
            "unsafe" => Some(TokenKind::Unsafe),
            "inline" => Some(TokenKind::Inline),
            "free" => Some(TokenKind::Free),
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
