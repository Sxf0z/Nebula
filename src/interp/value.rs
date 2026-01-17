use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::parser::ast::Param;
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),        
    Integer(i64),       
    Float(f64),         
    Bool(bool),         
    String(String),     
    Byte(u8),           
    Char(char),         
    Nil,                
    List(Vec<Value>),                    
    Map(HashMap<String, Value>),         
    Tuple(Vec<Value>),                   
    Set(Vec<Value>),                     
    Range(i64, i64, bool),               
    Function(Rc<FunctionValue>),
    Lambda(Rc<LambdaValue>),
    NativeFunction(NativeFn),
    Struct {
        name: String,
        fields: Vec<Value>,
    },
    Channel(Rc<RefCell<Vec<Value>>>),
}
#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub name: String,
    pub params: Vec<Param>,
    pub body: crate::parser::ast::FunctionBody,
    pub closure: Rc<RefCell<super::Environment>>,
    pub is_async: bool,
}
#[derive(Debug, Clone)]
pub struct LambdaValue {
    pub params: Vec<String>,
    pub body: crate::parser::ast::Expr,
    pub closure: Rc<RefCell<super::Environment>>,
}
#[derive(Clone)]
pub struct NativeFn {
    pub name: String,
    pub arity: Option<usize>,  
    pub func: fn(&[Value]) -> Result<Value, String>,
}
impl fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}
impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "nb",
            Value::Integer(_) => "int",
            Value::Float(_) => "fl",
            Value::Bool(_) => "bool",
            Value::String(_) => "wrd",
            Value::Byte(_) => "by",
            Value::Char(_) => "chr",
            Value::Nil => "nil",
            Value::List(_) => "lst",
            Value::Map(_) => "map",
            Value::Tuple(_) => "tup",
            Value::Set(_) => "set",
            Value::Range(_, _, _) => "range",
            Value::Function(_) => "fn",
            Value::Lambda(_) => "fn",
            Value::NativeFunction(_) => "fn",
            Value::Struct { .. } => "struct",
            Value::Channel(_) => "chan",
        }
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Nil => false,
            Value::Number(n) => *n != 0.0,
            Value::Integer(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(arr) => !arr.is_empty(),
            _ => true,
        }
    }
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Integer(n) => Some(*n as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            Value::Number(n) => Some(*n as i64),
            Value::Float(f) => Some(*f as i64),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn to_display_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            other => format!("{}", other),
        }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(true) => write!(f, "yes"),
            Value::Bool(false) => write!(f, "no"),
            Value::String(s) => write!(f, "{}", s),
            Value::Byte(b) => write!(f, "0x{:02X}", b),
            Value::Char(c) => write!(f, "{}", c),
            Value::Nil => write!(f, "nil"),
            Value::List(arr) => {
                write!(f, "lst(")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            Value::Map(m) => {
                write!(f, "map(")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, ")")
            }
            Value::Tuple(elements) => {
                write!(f, "(")?;
                for (i, v) in elements.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            Value::Set(elements) => {
                write!(f, "set(")?;
                for (i, v) in elements.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            Value::Range(start, end, inclusive) => {
                if *inclusive {
                    write!(f, "{}..{}", start, end)
                } else {
                    write!(f, "{}..<{}", start, end)
                }
            }
            Value::Function(func) => write!(f, "<fn {}>", func.name),
            Value::Lambda(_) => write!(f, "<lambda>"),
            Value::NativeFunction(nf) => write!(f, "<native fn {}>", nf.name),
            Value::Struct { name, fields } => {
                write!(f, "{}(", name)?;
                for (i, v) in fields.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            Value::Channel(_) => write!(f, "<chan>"),
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (Value::Number(a), Value::Integer(b)) => *a == *b as f64,
            (Value::Integer(a), Value::Number(b)) => *a as f64 == *b,
            _ => false,
        }
    }
}
