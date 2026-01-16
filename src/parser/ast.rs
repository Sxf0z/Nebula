//! AST (Abstract Syntax Tree) definitions for SpecterScript
//! Based on grammar.md specification

use crate::lexer::Span;

/// A complete SpecterScript program
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

/// Top-level items
#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    TypeAlias(TypeAlias),
    Module(Module),
    Use(Use),
    Statement(Stmt),
}

/// Function definition: fn name(params) = expr  OR  fn name(params) do ... end
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: FunctionBody,
    pub is_async: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum FunctionBody {
    Expression(Expr),      // fn add(a, b) = a + b
    Block(Vec<Stmt>),      // fn add(a, b) do ... end
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<Type>,
    pub default: Option<Expr>,
    pub variadic: bool,  // ...args
}

/// Struct definition: struct Name { field:type, ... }
#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
}

/// Struct field
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

/// Enum definition: enum Name { Variant1, Variant2, ... }
#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
    pub span: Span,
}

/// Type alias: type Name = OtherType
#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

/// Module declaration
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub span: Span,
}

/// Use/import statement
#[derive(Debug, Clone)]
pub struct Use {
    pub path: String,
    pub alias: Option<String>,
    pub span: Span,
}

/// Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Variable binding: fb x = 5
    Var {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    /// Constant binding: cn PI = 3.14
    Const {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    /// Assignment: x = 5
    Assignment {
        target: Expr,
        value: Expr,
    },
    /// Compound assignment: x += 5
    CompoundAssignment {
        target: Expr,
        op: CompoundOp,
        value: Expr,
    },
    /// If statement: if cond do ... elif ... else ... end
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        elif_branches: Vec<(Expr, Vec<Stmt>)>,
        else_block: Option<Vec<Stmt>>,
    },
    /// While loop: while cond do ... end
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    /// For loop: for i = 1, 10 do ... end  OR  for i = 1, 10, 2 do ... end
    For {
        var: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        body: Vec<Stmt>,
    },
    /// For-each loop: each x in lst do ... end
    Each {
        var: String,
        iterator: Expr,
        body: Vec<Stmt>,
    },
    /// Match statement
    Match {
        value: Expr,
        arms: Vec<MatchArm>,
    },
    /// Try/catch/finally
    Try {
        try_block: Vec<Stmt>,
        catch_var: Option<String>,
        catch_block: Option<Vec<Stmt>>,
        finally_block: Option<Vec<Stmt>>,
    },
    /// Return: -> value
    Return(Option<Expr>),
    /// Break
    Break,
    /// Continue
    Continue,
    /// Expression statement
    Expression(Expr),
}

#[derive(Debug, Clone, Copy)]
pub enum CompoundOp {
    Add,  // +=
    Sub,  // -=
    Mul,  // *=
    Div,  // /=
}

/// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
}

/// Patterns for matching
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,           // _
    Binding(String),    // x
    Literal(Literal),   // 42, "hello"
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    /// Literal value
    Literal(Literal),
    /// Variable reference
    Variable(String),
    /// Binary operation
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    /// Unary operation
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    /// Function call: name(args)
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    /// Method call: obj:method(args)
    MethodCall {
        receiver: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    /// Field access: obj.field
    Field {
        object: Box<Expr>,
        field: String,
    },
    /// Array/map index: arr[i]
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    /// Slice: arr[start:end]
    Slice {
        array: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
    /// Ternary: cond ? a : b
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    /// Lambda: (a, b) => a + b
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    /// List literal: lst(1, 2, 3)
    List(Vec<Expr>),
    /// Map literal: map("a": 1, "b": 2)
    Map(Vec<(Expr, Expr)>),
    /// Tuple: (1, "a")
    Tuple(Vec<Expr>),
    /// Range: 1..10 or 1..<10
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },
    /// Struct instantiation: Point(1, 2)
    StructInit {
        name: String,
        args: Vec<Expr>,
    },
    /// Length operator: #arr
    Length(Box<Expr>),
    /// Append: a << 5
    Append {
        list: Box<Expr>,
        value: Box<Expr>,
    },
    /// Await expression
    Await(Box<Expr>),
    /// Spawn expression
    Spawn(Box<Expr>),
    /// Error throw: err("message")
    Error(Box<Expr>),
    /// Assert: assert(cond, "msg")
    Assert {
        condition: Box<Expr>,
        message: Option<Box<Expr>>,
    },
    /// Channel send: ch <- value
    Send {
        channel: Box<Expr>,
        value: Box<Expr>,
    },
    /// Channel receive: <-ch
    Receive(Box<Expr>),
    /// Borrow: &value
    Borrow(Box<Expr>),
    /// Type cast: nb(value) or wrd(value)
    Cast {
        ty: Type,
        value: Box<Expr>,
    },
    /// Typeof
    TypeOf(Box<Expr>),
    /// Block expression (for grouping)
    Block(Vec<Stmt>),
    /// Nil literal
    Nil,
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Pow,
    // Comparison
    Eq, Ne, Lt, Gt, Le, Ge,
    // Logical (& and | are used for both logical and bitwise in grammar.md)
    And, Or,
    // Bitwise
    BitAnd, BitOr, BitXor, Shl, Shr,
}

impl BinaryOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Pow => "^",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
            BinaryOp::Le => "<=",
            BinaryOp::Ge => ">=",
            BinaryOp::And => "&",
            BinaryOp::Or => "|",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^|",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // !
    BitNot, // ~
}

/// Type annotations
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Nb,         // number (int or float)
    Int,        // integer
    Fl,         // float
    Wrd,        // string
    Bool,       // yes/no
    By,         // byte
    Chr,        // char
    Any,        // any
    Void,       // void
    Nil,        // nil
    
    // Collections
    Lst(Option<Box<Type>>),     // lst or lst[nb]
    Map(Option<Box<Type>>, Option<Box<Type>>),  // map or map[wrd, nb]
    Tup(Vec<Type>),             // (nb, wrd)
    Set(Option<Box<Type>>),     // set or set[nb]
    
    // Optional type
    Optional(Box<Type>),        // nb?
    
    // Named type (user-defined)
    Named(String),
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Nb | Type::Int | Type::Fl)
    }
}
