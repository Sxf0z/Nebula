//! Type checking passes

use crate::error::{SpectreError, SpectreResult};
use crate::parser::ast::*;
use super::types::{Ty, TypeEnv, TypeDef};
use super::infer::InferCtx;

/// The type checker
pub struct TypeChecker {
    env: TypeEnv,
    infer: InferCtx,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: TypeEnv::new(),
            infer: InferCtx::new(),
        }
    }

    /// Check an entire program
    pub fn check_program(&mut self, program: &Program) -> SpectreResult<()> {
        // First pass: collect type definitions
        for item in &program.items {
            match item {
                Item::Struct(s) => self.register_struct(s)?,
                Item::Enum(e) => self.register_enum(e)?,
                _ => {}
            }
        }

        // Second pass: check function signatures
        for item in &program.items {
            if let Item::Function(f) = item {
                self.register_function(f)?;
            }
        }

        // Third pass: check function bodies
        for item in &program.items {
            match item {
                Item::Function(f) => self.check_function(f)?,
                Item::Impl(i) => self.check_impl(i)?,
                Item::Statement(s) => { self.check_stmt(s)?; }
                _ => {}
            }
        }

        Ok(())
    }

    fn register_struct(&mut self, s: &Struct) -> SpectreResult<()> {
        let fields: Vec<_> = s.fields.iter()
            .map(|f| (f.name.clone(), Ty::from_ast(&f.ty)))
            .collect();
        self.env.define_type(s.name.clone(), TypeDef::Struct(fields));
        Ok(())
    }

    fn register_enum(&mut self, e: &Enum) -> SpectreResult<()> {
        let variants: Vec<_> = e.variants.iter()
            .map(|v| (v.name.clone(), v.fields.iter().map(Ty::from_ast).collect()))
            .collect();
        self.env.define_type(e.name.clone(), TypeDef::Enum(variants));
        Ok(())
    }

    fn register_function(&mut self, f: &Function) -> SpectreResult<()> {
        let param_types: Vec<_> = f.params.iter()
            .map(|p| Ty::from_ast(&p.ty))
            .collect();
        let return_type = f.return_type.as_ref()
            .map(Ty::from_ast)
            .unwrap_or(Ty::Unit);
        
        let fn_type = Ty::Function(param_types, Box::new(return_type));
        self.env.define(f.name.clone(), fn_type);
        Ok(())
    }

    fn check_function(&mut self, f: &Function) -> SpectreResult<()> {
        self.env.push_scope();

        // Bind parameters
        for param in &f.params {
            let ty = Ty::from_ast(&param.ty);
            self.env.define(param.name.clone(), ty);
        }

        // Check body
        let return_type = f.return_type.as_ref()
            .map(Ty::from_ast)
            .unwrap_or(Ty::Unit);

        for stmt in &f.body {
            self.check_stmt(stmt)?;
        }

        self.env.pop_scope();
        Ok(())
    }

    fn check_impl(&mut self, i: &Impl) -> SpectreResult<()> {
        for method in &i.methods {
            self.env.push_scope();
            
            // Bind 'self'
            let self_type = Ty::Generic(i.type_name.clone(), vec![]);
            self.env.define("self".to_string(), self_type);
            
            // Bind other parameters
            for param in &method.params {
                if param.name != "self" {
                    let ty = Ty::from_ast(&param.ty);
                    self.env.define(param.name.clone(), ty);
                }
            }

            for stmt in &method.body {
                self.check_stmt(stmt)?;
            }

            self.env.pop_scope();
        }
        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> SpectreResult<Ty> {
        match stmt {
            Stmt::Let { name, ty, value, .. } => {
                let value_type = self.check_expr(value)?;
                
                let declared_type = ty.as_ref()
                    .map(Ty::from_ast)
                    .unwrap_or_else(|| self.infer.fresh_var());

                if !self.infer.unify(&declared_type, &value_type) {
                    return Err(SpectreError::TypeMismatch {
                        expected: format!("{:?}", declared_type),
                        got: format!("{:?}", value_type),
                    });
                }

                let resolved = self.infer.resolve(&declared_type);
                self.env.define(name.clone(), resolved);
                Ok(Ty::Unit)
            }

            Stmt::Assignment { target, value } => {
                let target_type = self.check_expr(target)?;
                let value_type = self.check_expr(value)?;

                if !self.infer.unify(&target_type, &value_type) {
                    return Err(SpectreError::TypeMismatch {
                        expected: format!("{:?}", target_type),
                        got: format!("{:?}", value_type),
                    });
                }

                Ok(Ty::Unit)
            }

            Stmt::If { condition, then_block, elif_branches, else_block } => {
                let cond_type = self.check_expr(condition)?;
                if !self.infer.unify(&cond_type, &Ty::Bool) {
                    return Err(SpectreError::TypeMismatch {
                        expected: "bool".to_string(),
                        got: format!("{:?}", cond_type),
                    });
                }

                self.env.push_scope();
                for stmt in then_block {
                    self.check_stmt(stmt)?;
                }
                self.env.pop_scope();

                for (elif_cond, elif_body) in elif_branches {
                    let elif_cond_type = self.check_expr(elif_cond)?;
                    if !self.infer.unify(&elif_cond_type, &Ty::Bool) {
                        return Err(SpectreError::TypeMismatch {
                            expected: "bool".to_string(),
                            got: format!("{:?}", elif_cond_type),
                        });
                    }

                    self.env.push_scope();
                    for stmt in elif_body {
                        self.check_stmt(stmt)?;
                    }
                    self.env.pop_scope();
                }

                if let Some(else_body) = else_block {
                    self.env.push_scope();
                    for stmt in else_body {
                        self.check_stmt(stmt)?;
                    }
                    self.env.pop_scope();
                }

                Ok(Ty::Unit)
            }

            Stmt::While { condition, body } => {
                let cond_type = self.check_expr(condition)?;
                if !self.infer.unify(&cond_type, &Ty::Bool) {
                    return Err(SpectreError::TypeMismatch {
                        expected: "bool".to_string(),
                        got: format!("{:?}", cond_type),
                    });
                }

                self.env.push_scope();
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
                self.env.pop_scope();

                Ok(Ty::Unit)
            }

            Stmt::For { binding, iterator, body } => {
                let iter_type = self.check_expr(iterator)?;
                
                // For now, assume iterator yields i64 (range)
                // TODO: Proper iterator trait checking
                self.env.push_scope();
                self.env.define(binding.clone(), Ty::I64);
                
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
                self.env.pop_scope();

                Ok(Ty::Unit)
            }

            Stmt::Loop { body } => {
                self.env.push_scope();
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
                self.env.pop_scope();
                Ok(Ty::Unit)
            }

            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    self.check_expr(e)?;
                }
                Ok(Ty::Never)
            }

            Stmt::Break | Stmt::Continue => Ok(Ty::Never),

            Stmt::Expression(expr) => self.check_expr(expr),

            Stmt::Match { value, arms } => {
                let value_type = self.check_expr(value)?;
                
                for arm in arms {
                    // TODO: Check pattern against value_type
                    self.check_expr(&arm.body)?;
                }

                Ok(Ty::Unit)
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> SpectreResult<Ty> {
        match expr {
            Expr::Literal(lit) => Ok(self.literal_type(lit)),

            Expr::Variable(name) => {
                self.env.lookup(name)
                    .cloned()
                    .ok_or_else(|| SpectreError::UndefinedVariable { name: name.clone() })
            }

            Expr::Binary { left, op, right } => {
                let left_type = self.check_expr(left)?;
                let right_type = self.check_expr(right)?;

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if !self.infer.unify(&left_type, &right_type) {
                            return Err(SpectreError::TypeMismatch {
                                expected: format!("{:?}", left_type),
                                got: format!("{:?}", right_type),
                            });
                        }
                        Ok(self.infer.resolve(&left_type))
                    }
                    BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Gt | 
                    BinaryOp::Le | BinaryOp::Ge => {
                        if !self.infer.unify(&left_type, &right_type) {
                            return Err(SpectreError::TypeMismatch {
                                expected: format!("{:?}", left_type),
                                got: format!("{:?}", right_type),
                            });
                        }
                        Ok(Ty::Bool)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        if !self.infer.unify(&left_type, &Ty::Bool) {
                            return Err(SpectreError::TypeMismatch {
                                expected: "bool".to_string(),
                                got: format!("{:?}", left_type),
                            });
                        }
                        if !self.infer.unify(&right_type, &Ty::Bool) {
                            return Err(SpectreError::TypeMismatch {
                                expected: "bool".to_string(),
                                got: format!("{:?}", right_type),
                            });
                        }
                        Ok(Ty::Bool)
                    }
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor |
                    BinaryOp::Shl | BinaryOp::Shr => {
                        if !self.infer.unify(&left_type, &right_type) {
                            return Err(SpectreError::TypeMismatch {
                                expected: format!("{:?}", left_type),
                                got: format!("{:?}", right_type),
                            });
                        }
                        Ok(self.infer.resolve(&left_type))
                    }
                }
            }

            Expr::Unary { op, operand } => {
                let operand_type = self.check_expr(operand)?;
                match op {
                    UnaryOp::Neg => Ok(operand_type),
                    UnaryOp::Not => {
                        if !self.infer.unify(&operand_type, &Ty::Bool) {
                            return Err(SpectreError::TypeMismatch {
                                expected: "bool".to_string(),
                                got: format!("{:?}", operand_type),
                            });
                        }
                        Ok(Ty::Bool)
                    }
                    UnaryOp::BitNot => Ok(operand_type),
                }
            }

            Expr::Call { callee, args } => {
                let callee_type = self.check_expr(callee)?;
                
                match callee_type {
                    Ty::Function(params, ret) => {
                        if params.len() != args.len() {
                            return Err(SpectreError::InvalidOperation {
                                message: format!("Expected {} arguments, got {}", params.len(), args.len()),
                            });
                        }

                        for (param_type, arg) in params.iter().zip(args.iter()) {
                            let arg_type = self.check_expr(arg)?;
                            if !self.infer.unify(param_type, &arg_type) {
                                return Err(SpectreError::TypeMismatch {
                                    expected: format!("{:?}", param_type),
                                    got: format!("{:?}", arg_type),
                                });
                            }
                        }

                        Ok(*ret)
                    }
                    _ => Err(SpectreError::InvalidOperation {
                        message: "Cannot call non-function".to_string(),
                    })
                }
            }

            Expr::Array(elements) => {
                if elements.is_empty() {
                    return Ok(Ty::Array(Box::new(self.infer.fresh_var()), 0));
                }

                let first_type = self.check_expr(&elements[0])?;
                for elem in &elements[1..] {
                    let elem_type = self.check_expr(elem)?;
                    if !self.infer.unify(&first_type, &elem_type) {
                        return Err(SpectreError::TypeMismatch {
                            expected: format!("{:?}", first_type),
                            got: format!("{:?}", elem_type),
                        });
                    }
                }

                Ok(Ty::Array(Box::new(self.infer.resolve(&first_type)), elements.len()))
            }

            Expr::Tuple(elements) => {
                let types: Result<Vec<_>, _> = elements.iter()
                    .map(|e| self.check_expr(e))
                    .collect();
                Ok(Ty::Tuple(types?))
            }

            Expr::Unit => Ok(Ty::Unit),

            // Simplified handling for other expressions
            Expr::StructLiteral { name, fields } => {
                // TODO: Validate fields against struct definition
                Ok(Ty::Generic(name.clone(), vec![]))
            }

            Expr::Field { object, field } => {
                let obj_type = self.check_expr(object)?;
                // TODO: Look up field type from struct definition
                Ok(self.infer.fresh_var())
            }

            Expr::MethodCall { receiver, method, args } => {
                let _receiver_type = self.check_expr(receiver)?;
                // TODO: Look up method and check arguments
                Ok(self.infer.fresh_var())
            }

            Expr::Index { array, index } => {
                let array_type = self.check_expr(array)?;
                let index_type = self.check_expr(index)?;
                
                // Index must be integer
                if !index_type.is_integer() && !matches!(index_type, Ty::Var(_)) {
                    return Err(SpectreError::TypeMismatch {
                        expected: "integer".to_string(),
                        got: format!("{:?}", index_type),
                    });
                }

                match array_type {
                    Ty::Array(elem, _) | Ty::Slice(elem) => Ok(*elem),
                    _ => Ok(self.infer.fresh_var()),
                }
            }

            _ => Ok(self.infer.fresh_var()),
        }
    }

    fn literal_type(&self, lit: &Literal) -> Ty {
        match lit {
            Literal::Integer(_) => Ty::I64,
            Literal::Float(_) => Ty::F64,
            Literal::String(_) => Ty::String,
            Literal::Bool(_) => Ty::Bool,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
