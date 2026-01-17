use crate::lexer::Span;
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}
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
    Expression(Expr),      
    Block(Vec<Stmt>),      
}
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<Type>,
    pub default: Option<Expr>,
    pub variadic: bool,  
}
#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}
#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Use {
    pub path: String,
    pub alias: Option<String>,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Var {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Const {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Assignment {
        target: Expr,
        value: Expr,
    },
    CompoundAssignment {
        target: Expr,
        op: CompoundOp,
        value: Expr,
    },
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        elif_branches: Vec<(Expr, Vec<Stmt>)>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        body: Vec<Stmt>,
    },
    Each {
        var: String,
        iterator: Expr,
        body: Vec<Stmt>,
    },
    Match {
        value: Expr,
        arms: Vec<MatchArm>,
    },
    Try {
        try_block: Vec<Stmt>,
        catch_var: Option<String>,
        catch_block: Option<Vec<Stmt>>,
        finally_block: Option<Vec<Stmt>>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Expression(Expr),
}
#[derive(Debug, Clone, Copy)]
pub enum CompoundOp {
    Add,  
    Sub,  
    Mul,  
    Div,  
}
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
}
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,           
    Binding(String),    
    Literal(Literal),   
}
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    MethodCall {
        receiver: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    Field {
        object: Box<Expr>,
        field: String,
    },
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Slice {
        array: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Tuple(Vec<Expr>),
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },
    StructInit {
        name: String,
        args: Vec<Expr>,
    },
    Length(Box<Expr>),
    Append {
        list: Box<Expr>,
        value: Box<Expr>,
    },
    Await(Box<Expr>),
    Spawn(Box<Expr>),
    Error(Box<Expr>),
    Assert {
        condition: Box<Expr>,
        message: Option<Box<Expr>>,
    },
    Send {
        channel: Box<Expr>,
        value: Box<Expr>,
    },
    Receive(Box<Expr>),
    Borrow(Box<Expr>),
    Cast {
        ty: Type,
        value: Box<Expr>,
    },
    TypeOf(Box<Expr>),
    Block(Vec<Stmt>),
    Nil,
}
#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,    
    Not,    
    BitNot, 
}
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Nb,         
    Int,        
    Fl,         
    Wrd,        
    Bool,       
    By,         
    Chr,        
    Any,        
    Void,       
    Nil,        
    Lst(Option<Box<Type>>),     
    Map(Option<Box<Type>>, Option<Box<Type>>),  
    Tup(Vec<Type>),             
    Set(Option<Box<Type>>),     
    Optional(Box<Type>),        
    Named(String),
}
impl Type {
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Nb | Type::Int | Type::Fl)
    }
}
