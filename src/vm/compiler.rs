use crate::parser::ast::*;
use crate::interp::Value;
use crate::error::NebulaResult;
use super::{Chunk, OpCode};
struct CompilerScope {
    locals: Vec<String>,
    scope_depth: usize,
    local_depths: Vec<usize>,
}
impl CompilerScope {
    fn new() -> Self {
        Self {
            locals: Vec::with_capacity(16),
            scope_depth: 0,
            local_depths: Vec::with_capacity(16),
        }
    }
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    fn end_scope(&mut self) -> usize {
        self.scope_depth -= 1;
        let mut popped = 0;
        while !self.local_depths.is_empty() 
            && self.local_depths.last().copied().unwrap_or(0) > self.scope_depth 
        {
            self.locals.pop();
            self.local_depths.pop();
            popped += 1;
        }
        popped
    }
    fn add_local(&mut self, name: String) -> u8 {
        let slot = self.locals.len();
        self.locals.push(name);
        self.local_depths.push(self.scope_depth);
        slot as u8
    }
    fn resolve_local(&self, name: &str) -> Option<u8> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local == name {
                return Some(i as u8);
            }
        }
        None
    }
}
const BUILTIN_NAMES: [&str; 21] = [
    "log", "typeof", "sqrt", "abs", "len", "floor", "ceil", 
    "round", "pow", "sin", "cos", "tan", "exp", "ln", "get", 
    "rnd", "dbg", "now", "sleep", "str", "num"
];
pub struct Compiler {
    chunk: Chunk,
    scope: CompilerScope,
    global_names: Vec<String>,
    functions: Vec<super::CompiledFunction>,
}
impl Compiler {
    pub fn new() -> Self {
        let mut global_names = Vec::with_capacity(64);
        for name in BUILTIN_NAMES.iter() {
            global_names.push(name.to_string());
        }
        Self {
            chunk: Chunk::new(),
            scope: CompilerScope::new(),
            global_names,
            functions: Vec::new(),
        }
    }
    pub fn compile(&mut self, program: &Program) -> NebulaResult<Chunk> {
        for item in &program.items {
            self.compile_item(item)?;
        }
        self.emit(OpCode::PushNil, 0);
        self.emit(OpCode::Return, 0);
        Ok(std::mem::take(&mut self.chunk))
    }
    pub fn global_names(&self) -> &[String] {
        &self.global_names
    }
    pub fn functions(&self) -> &[super::CompiledFunction] {
        &self.functions
    }
    fn compile_item(&mut self, item: &Item) -> NebulaResult<()> {
        match item {
            Item::Statement(stmt) => self.compile_stmt(stmt),
            Item::Function(f) => self.compile_function_def(f),
            _ => Ok(()), 
        }
    }
    fn compile_function_def(&mut self, f: &Function) -> NebulaResult<()> {
        let mut func_compiler = Compiler::new();
        for param in &f.params {
            func_compiler.scope.add_local(param.name.clone());
        }
        match &f.body {
            crate::parser::ast::FunctionBody::Block(stmts) => {
                for stmt in stmts {
                    func_compiler.compile_stmt(stmt)?;
                }
            }
            crate::parser::ast::FunctionBody::Expression(expr) => {
                func_compiler.compile_expr(expr)?;
                func_compiler.emit(OpCode::Return, 0);
            }
        }
        func_compiler.emit(OpCode::PushNil, 0);
        func_compiler.emit(OpCode::Return, 0);
        let compiled = super::CompiledFunction {
            name: f.name.clone().into_boxed_str(),
            arity: f.params.len() as u8,
            local_count: func_compiler.scope.locals.len() as u8,
            chunk: func_compiler.chunk,
        };
        let func_idx = self.functions.len() as u8;
        self.functions.push(compiled);
        let global_idx = self.add_global(f.name.clone());
        self.emit(OpCode::Closure, 0);
        self.chunk.write_byte(func_idx, 0);
        self.emit(OpCode::DefineGlobal, 0);
        self.chunk.write_byte(global_idx, 0);
        Ok(())
    }
    fn compile_stmt(&mut self, stmt: &Stmt) -> NebulaResult<()> {
        let line = 0; 
        match stmt {
            Stmt::Var { name, value, .. } => {
                self.compile_expr(value)?;
                if self.scope.scope_depth > 0 {
                    self.scope.add_local(name.clone());
                } else {
                    let idx = self.add_global(name.clone());
                    self.emit(OpCode::DefineGlobal, line);
                    self.emit_byte(idx, line);
                }
                Ok(())
            }
            Stmt::Const { name, value, .. } => {
                self.compile_expr(value)?;
                if self.scope.scope_depth > 0 {
                    self.scope.add_local(name.clone());
                } else {
                    let idx = self.add_global(name.clone());
                    self.emit(OpCode::DefineGlobal, line);
                    self.emit_byte(idx, line);
                }
                Ok(())
            }
            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
                self.emit(OpCode::Pop, line);
                Ok(())
            }
            Stmt::If { condition, then_block, elif_branches, else_block } => {
                let mut end_jumps = Vec::new();
                self.compile_expr(condition)?;
                let then_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line); 
                self.compile_block(then_block)?;
                end_jumps.push(self.emit_jump(OpCode::Jump, line));
                self.patch_jump(then_jump);
                self.emit(OpCode::Pop, line); 
                for (elif_cond, elif_body) in elif_branches {
                    self.compile_expr(elif_cond)?;
                    let elif_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                    self.emit(OpCode::Pop, line); 
                    self.compile_block(elif_body)?;
                    end_jumps.push(self.emit_jump(OpCode::Jump, line));
                    self.patch_jump(elif_jump);
                    self.emit(OpCode::Pop, line); 
                }
                if let Some(else_body) = else_block {
                    self.compile_block(else_body)?;
                }
                for jump in end_jumps {
                    self.patch_jump(jump);
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                let loop_start = self.chunk.len();
                self.emit(OpCode::CheckIterLimit, line);
                self.compile_expr(condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line);
                self.compile_block(body)?;
                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                self.emit(OpCode::Pop, line);
                Ok(())
            }
            Stmt::Return(value) => {
                if let Some(expr) = value {
                    self.compile_expr(expr)?;
                } else {
                    self.emit(OpCode::PushNil, line);
                }
                self.emit(OpCode::Return, line);
                Ok(())
            }
            Stmt::For { var, start, end, step, body } => {
                self.scope.begin_scope();
                self.compile_expr(start)?;
                let var_slot = self.scope.add_local(var.clone());
                let loop_start = self.chunk.len();
                self.emit(OpCode::CheckIterLimit, line);
                self.emit(OpCode::LoadLocal, line);
                self.emit_byte(var_slot, line);
                self.compile_expr(end)?;
                self.emit(OpCode::Le, line);
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line); 
                self.compile_block(body)?;
                self.emit(OpCode::LoadLocal, line);
                self.emit_byte(var_slot, line);
                if let Some(step_expr) = step {
                    self.compile_expr(step_expr)?;
                } else {
                    let idx = self.chunk.add_constant(Value::Integer(1));
                    self.emit(OpCode::PushConst, line);
                    self.emit_byte(idx, line);
                }
                self.emit(OpCode::Add, line);
                self.emit(OpCode::StoreLocal, line);
                self.emit_byte(var_slot, line);
                self.emit(OpCode::Pop, line); 
                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                self.emit(OpCode::Pop, line); 
                self.scope.end_scope();
                self.emit(OpCode::Pop, line); 
                Ok(())
            }
            Stmt::Each { var, iterator, body } => {
                self.scope.begin_scope();
                self.compile_expr(iterator)?;
                self.emit(OpCode::IterInit, line);
                self.emit(OpCode::PushNil, line); 
                let var_slot = self.scope.add_local(var.clone());
                let loop_start = self.chunk.len();
                self.emit(OpCode::CheckIterLimit, line);
                let exit_jump = self.emit_jump(OpCode::IterNext, line);
                self.emit(OpCode::StoreLocal, line);
                self.emit_byte(var_slot, line);
                self.emit(OpCode::Pop, line);
                self.compile_block(body)?;
                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                let pops = self.scope.end_scope();
                for _ in 0..pops {
                    self.emit(OpCode::Pop, line);
                }
                self.emit(OpCode::Pop, line); 
                Ok(())
            }
            Stmt::Assignment { target, value } => {
                self.compile_expr(value)?;
                if let Expr::Variable(name) = target {
                    if let Some(slot) = self.scope.resolve_local(name) {
                        match slot {
                            0 => self.emit(OpCode::StoreLocal0, line),
                            1 => self.emit(OpCode::StoreLocal1, line),
                            2 => self.emit(OpCode::StoreLocal2, line),
                            _ => {
                                self.emit(OpCode::StoreLocal, line);
                                self.emit_byte(slot, line);
                            }
                        }
                        self.emit(OpCode::Pop, line);
                    } else if let Some(idx) = self.global_names.iter().position(|n| n == name) {
                        let idx = idx as u8;
                        match idx {
                            21 => self.emit(OpCode::StoreGlobal0, line),
                            22 => self.emit(OpCode::StoreGlobal1, line),
                            23 => self.emit(OpCode::StoreGlobal2, line),
                            _ => {
                                self.emit(OpCode::StoreGlobal, line);
                                self.emit_byte(idx, line);
                            }
                        }
                        self.emit(OpCode::Pop, line);
                    } else {
                        if self.scope.scope_depth > 0 {
                            self.scope.add_local(name.clone());
                        } else {
                            let idx = self.add_global(name.clone());
                            self.emit(OpCode::DefineGlobal, line);
                            self.emit_byte(idx, line);
                        }
                    }
                }
                Ok(())
            }
            _ => {
                Ok(())
            }
        }
    }
    fn compile_block(&mut self, stmts: &[Stmt]) -> NebulaResult<()> {
        self.scope.begin_scope();
        for stmt in stmts {
            self.compile_stmt(stmt)?;
        }
        let pops = self.scope.end_scope();
        for _ in 0..pops {
            self.emit(OpCode::Pop, 0);
        }
        Ok(())
    }
    fn compile_expr(&mut self, expr: &Expr) -> NebulaResult<()> {
        let line = 0; 
        match expr {
            Expr::Literal(lit) => {
                match lit {
                    Literal::Integer(n) => {
                        let idx = self.chunk.add_constant(Value::Integer(*n));
                        self.emit(OpCode::PushConst, line);
                        self.emit_byte(idx, line);
                    }
                    Literal::Float(f) => {
                        let idx = self.chunk.add_constant(Value::Number(*f));
                        self.emit(OpCode::PushConst, line);
                        self.emit_byte(idx, line);
                    }
                    Literal::String(s) => {
                        let idx = self.chunk.add_constant(Value::String(s.clone()));
                        self.emit(OpCode::PushConst, line);
                        self.emit_byte(idx, line);
                    }
                    Literal::Bool(b) => {
                        self.emit(if *b { OpCode::PushTrue } else { OpCode::PushFalse }, line);
                    }
                }
                Ok(())
            }
            Expr::Variable(name) => {
                if let Some(slot) = self.scope.resolve_local(name) {
                    match slot {
                        0 => self.emit(OpCode::LoadLocal0, line),
                        1 => self.emit(OpCode::LoadLocal1, line),
                        2 => self.emit(OpCode::LoadLocal2, line),
                        _ => {
                            self.emit(OpCode::LoadLocal, line);
                            self.emit_byte(slot, line);
                        }
                    }
                } else {
                    let idx = self.resolve_global(name);
                    match idx {
                        21 => self.emit(OpCode::LoadGlobal0, line),
                        22 => self.emit(OpCode::LoadGlobal1, line),
                        23 => self.emit(OpCode::LoadGlobal2, line),
                        _ => {
                            self.emit(OpCode::LoadGlobal, line);
                            self.emit_byte(idx, line);
                        }
                    }
                }
                Ok(())
            }
            Expr::Binary { left, op, right } => {
                if let Some(result) = self.try_fold_binary(left, op, right)? {
                    let idx = self.chunk.add_constant(result);
                    self.emit(OpCode::PushConst, line);
                    self.emit_byte(idx, line);
                } else {
                    self.compile_expr(left)?;
                    self.compile_expr(right)?;
                    self.emit_binary_op(op, line);
                }
                Ok(())
            }
            Expr::Unary { op, operand } => {
                self.compile_expr(operand)?;
                match op {
                    UnaryOp::Neg => self.emit(OpCode::Neg, line),
                    UnaryOp::Not => self.emit(OpCode::Not, line),
                    _ => {}
                }
                Ok(())
            }
            Expr::Call { callee, args } => {
                self.compile_expr(callee)?;
                for arg in args {
                    self.compile_expr(arg)?;
                }
                self.emit(OpCode::Call, line);
                self.emit_byte(args.len() as u8, line);
                Ok(())
            }
            Expr::List(items) => {
                for item in items {
                    self.compile_expr(item)?;
                }
                self.emit(OpCode::List, line);
                self.emit_byte(items.len() as u8, line);
                Ok(())
            }
            _ => {
                Ok(())
            }
        }
    }
    fn emit(&mut self, op: OpCode, line: usize) {
        self.chunk.write_op(op, line);
    }
    fn emit_byte(&mut self, byte: u8, line: usize) {
        self.chunk.write_byte(byte, line);
    }
    fn emit_jump(&mut self, op: OpCode, line: usize) -> usize {
        self.emit(op, line);
        self.chunk.write_u16(0xffff, line);
        self.chunk.len() - 2
    }
    fn patch_jump(&mut self, offset: usize) {
        self.chunk.patch_jump(offset);
    }
    fn emit_loop(&mut self, loop_start: usize, line: usize) {
        self.emit(OpCode::Loop, line);
        let offset = self.chunk.len().saturating_sub(loop_start) + 2;
        let offset = offset.min(u16::MAX as usize);
        self.chunk.write_u16(offset as u16, line);
    }
    fn emit_binary_op(&mut self, op: &BinaryOp, line: usize) {
        match op {
            BinaryOp::Add => self.emit(OpCode::Add, line),
            BinaryOp::Sub => self.emit(OpCode::Sub, line),
            BinaryOp::Mul => self.emit(OpCode::Mul, line),
            BinaryOp::Div => self.emit(OpCode::Div, line),
            BinaryOp::Mod => self.emit(OpCode::Mod, line),
            BinaryOp::Pow => self.emit(OpCode::Pow, line),
            BinaryOp::Eq => self.emit(OpCode::Eq, line),
            BinaryOp::Ne => self.emit(OpCode::Ne, line),
            BinaryOp::Lt => self.emit(OpCode::Lt, line),
            BinaryOp::Gt => self.emit(OpCode::Gt, line),
            BinaryOp::Le => self.emit(OpCode::Le, line),
            BinaryOp::Ge => self.emit(OpCode::Ge, line),
            _ => {}
        }
    }
    fn add_global(&mut self, name: String) -> u8 {
        for (i, n) in self.global_names.iter().enumerate() {
            if n == &name {
                return i as u8;
            }
        }
        let idx = self.global_names.len() as u8;
        self.global_names.push(name);
        idx
    }
    fn resolve_global(&mut self, name: &str) -> u8 {
        for (i, n) in self.global_names.iter().enumerate() {
            if n == name {
                return i as u8;
            }
        }
        self.add_global(name.to_string())
    }
    fn try_fold_binary(&self, left: &Expr, op: &BinaryOp, right: &Expr) -> NebulaResult<Option<Value>> {
        let lval = match self.extract_number(left) {
            Some(v) => v,
            None => return Ok(None),
        };
        let rval = match self.extract_number(right) {
            Some(v) => v,
            None => return Ok(None),
        };
        let result = match op {
            BinaryOp::Add => lval + rval,
            BinaryOp::Sub => lval - rval,
            BinaryOp::Mul => lval * rval,
            BinaryOp::Div => {
                if rval == 0.0 {
                    return Err(crate::error::NebulaError::coded(
                        crate::error::ErrorCode::E040,
                        "division by zero in constant expression"
                    ));
                }
                lval / rval
            }
            BinaryOp::Mod => {
                if rval == 0.0 {
                    return Err(crate::error::NebulaError::coded(
                        crate::error::ErrorCode::E040,
                        "modulo by zero in constant expression"
                    ));
                }
                lval % rval
            }
            BinaryOp::Pow => lval.powf(rval),
            _ => return Ok(None), 
        };
        if result.fract() == 0.0 && result.abs() < (i64::MAX as f64) {
            Ok(Some(Value::Integer(result as i64)))
        } else {
            Ok(Some(Value::Number(result)))
        }
    }
    fn extract_number(&self, expr: &Expr) -> Option<f64> {
        match expr {
            Expr::Literal(Literal::Integer(n)) => Some(*n as f64),
            Expr::Literal(Literal::Float(f)) => Some(*f),
            Expr::Binary { left, op, right } => {
                self.try_fold_binary(left, op, right).ok()?.and_then(|v| match v {
                    Value::Integer(n) => Some(n as f64),
                    Value::Number(f) => Some(f),
                    _ => None,
                })
            }
            Expr::Unary { op: UnaryOp::Neg, operand } => {
                self.extract_number(operand).map(|n| -n)
            }
            _ => None,
        }
    }
}
impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
