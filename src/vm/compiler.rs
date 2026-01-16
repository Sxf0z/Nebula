//! AST to bytecode compiler
//!
//! Compiles SpecterScript AST into bytecode chunks.

use crate::parser::ast::*;
use crate::interp::Value;
use crate::error::SpectreResult;
use super::{Chunk, OpCode};

/// Compiler state for a single function/scope
struct CompilerScope {
    /// Local variable names to slot indices
    locals: Vec<String>,
    /// Scope depth (0 = function level)
    scope_depth: usize,
    /// Local variable scope depths
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

/// Builtin names that get reserved slots (must match VM's BUILTIN_NAMES)
const BUILTIN_NAMES: [&str; 21] = [
    "log", "typeof", "sqrt", "abs", "len", "floor", "ceil", 
    "round", "pow", "sin", "cos", "tan", "exp", "ln", "get", 
    "rnd", "dbg", "now", "sleep", "str", "num"
];

/// Bytecode compiler
pub struct Compiler {
    /// Current chunk being compiled
    chunk: Chunk,
    /// Compiler scope
    scope: CompilerScope,
    /// Global variable names (index -> name)
    global_names: Vec<String>,
    /// Compiled functions (name -> CompiledFunction)
    functions: Vec<super::CompiledFunction>,
}

impl Compiler {
    pub fn new() -> Self {
        // Pre-register builtins at fixed indices to match VM
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

    /// Compile a program to bytecode
    pub fn compile(&mut self, program: &Program) -> SpectreResult<Chunk> {
        for item in &program.items {
            self.compile_item(item)?;
        }
        
        // Implicit return nil at end
        self.emit(OpCode::PushNil, 0);
        self.emit(OpCode::Return, 0);
        
        Ok(std::mem::take(&mut self.chunk))
    }

    /// Get the global name table for VM execution
    pub fn global_names(&self) -> &[String] {
        &self.global_names
    }
    
    /// Get compiled functions
    pub fn functions(&self) -> &[super::CompiledFunction] {
        &self.functions
    }

    fn compile_item(&mut self, item: &Item) -> SpectreResult<()> {
        match item {
            Item::Statement(stmt) => self.compile_stmt(stmt),
            Item::Function(f) => self.compile_function_def(f),
            _ => Ok(()), // TODO: structs, enums, impls
        }
    }

    fn compile_function_def(&mut self, f: &Function) -> SpectreResult<()> {
        // Create a new compiler for the function body
        let mut func_compiler = Compiler::new();
        
        // Add parameters as local variables
        for param in &f.params {
            func_compiler.scope.add_local(param.name.clone());
        }
        
        // Compile function body
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
        
        // Emit implicit return nil if no explicit return
        func_compiler.emit(OpCode::PushNil, 0);
        func_compiler.emit(OpCode::Return, 0);
        
        // Create CompiledFunction
        let compiled = super::CompiledFunction {
            name: f.name.clone().into_boxed_str(),
            arity: f.params.len() as u8,
            local_count: func_compiler.scope.locals.len() as u8,
            chunk: func_compiler.chunk,
        };
        
        // Store in functions list
        let func_idx = self.functions.len() as u8;
        self.functions.push(compiled);
        
        // Register function name as global
        let global_idx = self.add_global(f.name.clone());
        
        // Emit: Closure func_idx, DefineGlobal global_idx
        self.emit(OpCode::Closure, 0);
        self.chunk.write_byte(func_idx, 0);
        self.emit(OpCode::DefineGlobal, 0);
        self.chunk.write_byte(global_idx, 0);
        
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> SpectreResult<()> {
        let line = 0; // TODO: get line from span

        match stmt {
            Stmt::Var { name, value, .. } => {
                self.compile_expr(value)?;
                
                if self.scope.scope_depth > 0 {
                    // Local variable
                    self.scope.add_local(name.clone());
                } else {
                    // Global variable
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
                // Compile: if cond then body [elif cond then body]* [else body] end
                // 
                // Structure:
                //   condition
                //   JUMP_IF_FALSE -> elif1/else/end
                //   POP condition
                //   then_body
                //   JUMP -> end
                //   [repeat for each elif]
                //   else_body (or nothing)
                // end:
                
                let mut end_jumps = Vec::new();
                
                // Compile main condition
                self.compile_expr(condition)?;
                let then_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line); // Pop condition
                
                // Then block
                self.compile_block(then_block)?;
                end_jumps.push(self.emit_jump(OpCode::Jump, line));
                
                // Patch then_jump to here (elif/else/end)
                self.patch_jump(then_jump);
                self.emit(OpCode::Pop, line); // Pop condition (false path)
                
                // Handle elif branches
                for (elif_cond, elif_body) in elif_branches {
                    self.compile_expr(elif_cond)?;
                    let elif_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                    self.emit(OpCode::Pop, line); // Pop condition
                    
                    self.compile_block(elif_body)?;
                    end_jumps.push(self.emit_jump(OpCode::Jump, line));
                    
                    self.patch_jump(elif_jump);
                    self.emit(OpCode::Pop, line); // Pop condition (false path)
                }
                
                // Else block
                if let Some(else_body) = else_block {
                    self.compile_block(else_body)?;
                }
                
                // Patch all end jumps to here
                for jump in end_jumps {
                    self.patch_jump(jump);
                }
                
                Ok(())
            }

            Stmt::While { condition, body } => {
                let loop_start = self.chunk.len();
                
                // Check iteration limit
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
                // For loop: for i = start, end [, step] do body end
                // 
                // Each iteration we need to:
                // 1. Re-init the loop var to start (for nested loops)
                // 2. Check condition
                // 3. Execute body
                // 4. Increment
                
                // Reserve slot for loop var at current scope level
                self.scope.begin_scope();
                
                // Push initial value (this allocates stack slot)
                self.compile_expr(start)?;
                let var_slot = self.scope.add_local(var.clone());
                
                let loop_start = self.chunk.len();
                
                // Check iteration limit
                self.emit(OpCode::CheckIterLimit, line);
                
                // Load loop variable
                self.emit(OpCode::LoadLocal, line);
                self.emit_byte(var_slot, line);
                
                // Evaluate end
                self.compile_expr(end)?;
                
                // Compare: var <= end
                self.emit(OpCode::Le, line);
                
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line); // Pop comparison result
                
                // Compile body statements in a nested scope
                // This ensures locals declared inside the loop are popped each iteration
                self.compile_block(body)?;
                
                // Increment: var += step
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
                self.emit(OpCode::Pop, line); // Pop stored value
                
                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                self.emit(OpCode::Pop, line); // Pop false comparison
                
                // End scope and pop loop variable
                self.scope.end_scope();
                self.emit(OpCode::Pop, line); // Pop loop var
                
                Ok(())
            }

            Stmt::Each { var, iterator, body } => {
                // Each loop: each x in collection do body end
                // Compile as:
                //   evaluate collection
                //   ITER_INIT
                //   loop:
                //     check iteration limit  
                //     ITER_NEXT -> jump if done
                //     store to x (local)
                //     body
                //     jump -> loop
                
                self.scope.begin_scope();
                
                // Evaluate iterator expression
                self.compile_expr(iterator)?;
                
                // Initialize iterator
                self.emit(OpCode::IterInit, line);
                
                // Add loop variable
                self.emit(OpCode::PushNil, line); // Placeholder for loop var
                let var_slot = self.scope.add_local(var.clone());
                
                let loop_start = self.chunk.len();
                
                // Check iteration limit
                self.emit(OpCode::CheckIterLimit, line);
                
                // Get next value or exit
                let exit_jump = self.emit_jump(OpCode::IterNext, line);
                
                // Store next value to loop variable
                self.emit(OpCode::StoreLocal, line);
                self.emit_byte(var_slot, line);
                self.emit(OpCode::Pop, line);
                
                // Compile body
                self.compile_block(body)?;
                
                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                
                // Pop the loop variable and iterator
                let pops = self.scope.end_scope();
                for _ in 0..pops {
                    self.emit(OpCode::Pop, line);
                }
                self.emit(OpCode::Pop, line); // Pop iterator
                
                Ok(())
            }

            Stmt::Assignment { target, value } => {
                self.compile_expr(value)?;
                
                // Handle simple variable assignment
                if let Expr::Variable(name) = target {
                    if let Some(slot) = self.scope.resolve_local(name) {
                        // Use specialized opcodes for common slots
                        match slot {
                            0 => self.emit(OpCode::StoreLocal0, line),
                            1 => self.emit(OpCode::StoreLocal1, line),
                            2 => self.emit(OpCode::StoreLocal2, line),
                            _ => {
                                self.emit(OpCode::StoreLocal, line);
                                self.emit_byte(slot, line);
                            }
                        }
                    } else {
                        let idx = self.resolve_global(name);
                        self.emit(OpCode::StoreGlobal, line);
                        self.emit_byte(idx, line);
                    }
                    // Pop the stored value - assignment is a statement, not expression
                    self.emit(OpCode::Pop, line);
                }
                Ok(())
            }

            _ => {
                // TODO: Match, Try, etc.
                Ok(())
            }
        }
    }

    fn compile_block(&mut self, stmts: &[Stmt]) -> SpectreResult<()> {
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

    fn compile_expr(&mut self, expr: &Expr) -> SpectreResult<()> {
        let line = 0; // TODO: get from span

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
                    // Use specialized opcodes for common slots (no operand needed)
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
                    self.emit(OpCode::LoadGlobal, line);
                    self.emit_byte(idx, line);
                }
                Ok(())
            }

            Expr::Binary { left, op, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                
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
                    _ => {} // TODO: other ops
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
                // TODO: other expressions
                Ok(())
            }
        }
    }

    // === Emit helpers ===

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
        // Saturate to max if overflow
        let offset = offset.min(u16::MAX as usize);
        self.chunk.write_u16(offset as u16, line);
    }

    fn add_global(&mut self, name: String) -> u8 {
        // Check if already exists
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
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
