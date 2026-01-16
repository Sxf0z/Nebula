//! Type representations for type checking

use std::collections::HashMap;
use crate::parser::ast::Type as AstType;

/// Internal type representation for type checking
#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    // Primitives
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    Bool,
    Str,
    String,
    Unit,
    
    // Never type (for return/break)
    Never,
    
    // Composite
    Array(Box<Ty>, usize),
    Slice(Box<Ty>),
    Tuple(Vec<Ty>),
    
    // User-defined
    Struct(String, Vec<(String, Ty)>),
    Enum(String, Vec<(String, Vec<Ty>)>),
    
    // Generic/inference
    Var(usize), // Type variable for inference
    Generic(String, Vec<Ty>),
    
    // Function type
    Function(Vec<Ty>, Box<Ty>),
    
    // Error placeholder
    Error,
}

impl Ty {
    pub fn from_ast(ast_type: &AstType) -> Self {
        match ast_type {
            AstType::I8 => Ty::I8,
            AstType::I16 => Ty::I16,
            AstType::I32 => Ty::I32,
            AstType::I64 => Ty::I64,
            AstType::U8 => Ty::U8,
            AstType::U16 => Ty::U16,
            AstType::U32 => Ty::U32,
            AstType::U64 => Ty::U64,
            AstType::F32 => Ty::F32,
            AstType::F64 => Ty::F64,
            AstType::Bool => Ty::Bool,
            AstType::Str => Ty::Str,
            AstType::String => Ty::String,
            AstType::Unit => Ty::Unit,
            AstType::SelfType => Ty::Error, // Resolved later
            AstType::Array(elem, size) => Ty::Array(Box::new(Ty::from_ast(elem)), *size),
            AstType::Slice(elem) => Ty::Slice(Box::new(Ty::from_ast(elem))),
            AstType::Tuple(types) => Ty::Tuple(types.iter().map(Ty::from_ast).collect()),
            AstType::Named(name) => Ty::Generic(name.clone(), vec![]),
            AstType::Generic(name, args) => {
                Ty::Generic(name.clone(), args.iter().map(Ty::from_ast).collect())
            }
            AstType::Infer => Ty::Error,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self,
            Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 |
            Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 |
            Ty::F32 | Ty::F64
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(self,
            Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 |
            Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64
        )
    }

    pub fn is_signed(&self) -> bool {
        matches!(self, Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64)
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Ty::F32 | Ty::F64)
    }
}

/// Type environment for scoped type checking
#[derive(Debug, Clone)]
pub struct TypeEnv {
    scopes: Vec<HashMap<String, Ty>>,
    type_defs: HashMap<String, TypeDef>,
}

#[derive(Debug, Clone)]
pub enum TypeDef {
    Struct(Vec<(String, Ty)>),
    Enum(Vec<(String, Vec<Ty>)>),
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            type_defs: HashMap::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, ty: Ty) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Ty> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }

    pub fn define_type(&mut self, name: String, def: TypeDef) {
        self.type_defs.insert(name, def);
    }

    pub fn lookup_type(&self, name: &str) -> Option<&TypeDef> {
        self.type_defs.get(name)
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}
