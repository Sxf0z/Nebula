use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::error::{NebulaError, NebulaResult, ErrorCode};
use crate::parser::ast::*;
use super::value::{Value, FunctionValue, LambdaValue, NativeFn};
use super::env::Environment;
enum ControlFlow {
    Return(Value),
    Break,
    Continue,
}
type EvalResult = Result<Value, EvalError>;
enum EvalError {
    Error(NebulaError),
    Control(ControlFlow),
}
impl From<NebulaError> for EvalError {
    fn from(e: NebulaError) -> Self {
        EvalError::Error(e)
    }
}
const MAX_RECURSION_DEPTH: usize = 50;
const MAX_ITERATIONS: usize = 1_000_000;
pub struct Interpreter {
    global: Rc<RefCell<Environment>>,
    current: Rc<RefCell<Environment>>,
    structs: HashMap<String, Vec<String>>,
    recursion_depth: usize,
    iteration_count: usize,
}
impl Interpreter {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(Environment::new()));
        {
            let mut env = global.borrow_mut();
            env.define("log".to_string(), Value::NativeFunction(NativeFn {
                name: "log".to_string(),
                arity: None, 
                func: |args| {
                    let output: Vec<_> = args.iter().map(|a| a.to_display_string()).collect();
                    println!("{}", output.join(" "));
                    Ok(Value::Nil)
                },
            }));
            env.define("get".to_string(), Value::NativeFunction(NativeFn {
                name: "get".to_string(),
                arity: Some(0),
                func: |_args| {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
                    Ok(Value::String(line.trim().to_string()))
                },
            }));
            env.define("typeof".to_string(), Value::NativeFunction(NativeFn {
                name: "typeof".to_string(),
                arity: Some(1),
                func: |args| {
                    Ok(Value::String(args[0].type_name().to_string()))
                },
            }));
            env.define("sqrt".to_string(), Value::NativeFunction(NativeFn {
                name: "sqrt".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("sqrt requires number")?;
                    Ok(Value::Number(n.sqrt()))
                },
            }));
            env.define("abs".to_string(), Value::NativeFunction(NativeFn {
                name: "abs".to_string(),
                arity: Some(1),
                func: |args| {
                    match &args[0] {
                        Value::Number(n) => Ok(Value::Number(n.abs())),
                        Value::Integer(n) => Ok(Value::Integer(n.abs())),
                        Value::Float(f) => Ok(Value::Float(f.abs())),
                        _ => Err("abs requires number".to_string()),
                    }
                },
            }));
            env.define("sin".to_string(), Value::NativeFunction(NativeFn {
                name: "sin".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("sin requires number")?;
                    Ok(Value::Number(n.sin()))
                },
            }));
            env.define("cos".to_string(), Value::NativeFunction(NativeFn {
                name: "cos".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("cos requires number")?;
                    Ok(Value::Number(n.cos()))
                },
            }));
            env.define("tan".to_string(), Value::NativeFunction(NativeFn {
                name: "tan".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("tan requires number")?;
                    Ok(Value::Number(n.tan()))
                },
            }));
            env.define("floor".to_string(), Value::NativeFunction(NativeFn {
                name: "floor".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("floor requires number")?;
                    Ok(Value::Number(n.floor()))
                },
            }));
            env.define("ceil".to_string(), Value::NativeFunction(NativeFn {
                name: "ceil".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("ceil requires number")?;
                    Ok(Value::Number(n.ceil()))
                },
            }));
            env.define("round".to_string(), Value::NativeFunction(NativeFn {
                name: "round".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("round requires number")?;
                    Ok(Value::Number(n.round()))
                },
            }));
            env.define("pow".to_string(), Value::NativeFunction(NativeFn {
                name: "pow".to_string(),
                arity: Some(2),
                func: |args| {
                    let base = args[0].as_number().ok_or("pow requires number")?;
                    let exp = args[1].as_number().ok_or("pow requires number")?;
                    Ok(Value::Number(base.powf(exp)))
                },
            }));
            env.define("exp".to_string(), Value::NativeFunction(NativeFn {
                name: "exp".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("exp requires number")?;
                    Ok(Value::Number(n.exp()))
                },
            }));
            env.define("ln".to_string(), Value::NativeFunction(NativeFn {
                name: "ln".to_string(),
                arity: Some(1),
                func: |args| {
                    let n = args[0].as_number().ok_or("ln requires number")?;
                    Ok(Value::Number(n.ln()))
                },
            }));
            env.define("len".to_string(), Value::NativeFunction(NativeFn {
                name: "len".to_string(),
                arity: Some(1),
                func: |args| {
                    match &args[0] {
                        Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                        Value::List(l) => Ok(Value::Integer(l.len() as i64)),
                        Value::Map(m) => Ok(Value::Integer(m.len() as i64)),
                        Value::Tuple(t) => Ok(Value::Integer(t.len() as i64)),
                        _ => Err(format!("len() requires collection or string, got {}", args[0].type_name())),
                    }
                },
            }));
            env.define("rnd".to_string(), Value::NativeFunction(NativeFn {
                name: "rnd".to_string(),
                arity: Some(0),
                func: |_args| {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let seed = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .subsec_nanos() as f64;
                    Ok(Value::Number((seed / 1_000_000_000.0) % 1.0))
                },
            }));
            env.define("dbg".to_string(), Value::NativeFunction(NativeFn {
                name: "dbg".to_string(),
                arity: None,
                func: |args| {
                    for arg in args {
                        eprintln!("[dbg] {:?}", arg);
                    }
                    Ok(Value::Nil)
                },
            }));
            env.define("chan".to_string(), Value::NativeFunction(NativeFn {
                name: "chan".to_string(),
                arity: Some(0),
                func: |_args| {
                    Ok(Value::Channel(Rc::new(RefCell::new(Vec::new()))))
                },
            }));
            env.define("now".to_string(), Value::NativeFunction(NativeFn {
                name: "now".to_string(),
                arity: Some(0),
                func: |_args| {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let ms = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as f64;
                    Ok(Value::Number(ms))
                },
            }));
            env.define("sleep".to_string(), Value::NativeFunction(NativeFn {
                name: "sleep".to_string(),
                arity: Some(1),
                func: |args| {
                    let ms = args[0].as_number().ok_or("sleep requires number (milliseconds)")?;
                    if ms > 0.0 {
                        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
                    }
                    Ok(Value::Nil)
                },
            }));
            env.define("str".to_string(), Value::NativeFunction(NativeFn {
                name: "str".to_string(),
                arity: Some(1),
                func: |args| {
                    Ok(Value::String(args[0].to_display_string()))
                },
            }));
            env.define("num".to_string(), Value::NativeFunction(NativeFn {
                name: "num".to_string(),
                arity: Some(1),
                func: |args| {
                    match &args[0] {
                        Value::Number(n) => Ok(Value::Number(*n)),
                        Value::Integer(n) => Ok(Value::Number(*n as f64)),
                        Value::Float(f) => Ok(Value::Number(*f)),
                        Value::String(s) => {
                            s.parse::<f64>()
                                .map(Value::Number)
                                .map_err(|_| format!("Cannot convert '{}' to number", s))
                        }
                        Value::Bool(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
                        _ => Err(format!("Cannot convert {} to number", args[0].type_name())),
                    }
                },
            }));
        }
        let current = Rc::clone(&global);
        Self {
            global,
            current,
            structs: HashMap::new(),
            recursion_depth: 0,
            iteration_count: 0,
        }
    }
    pub fn reset_scope(&mut self) {
        self.current = Rc::clone(&self.global);
    }
    pub fn interpret(&mut self, program: &Program) -> NebulaResult<Value> {
        let mut result = Value::Nil;
        for item in &program.items {
            match item {
                Item::Struct(s) => {
                    let fields: Vec<_> = s.fields.iter().map(|f| f.name.clone()).collect();
                    self.structs.insert(s.name.clone(), fields);
                }
                Item::Function(f) => {
                    self.define_function(f);
                }
                _ => {}
            }
        }
        for item in &program.items {
            if let Item::Statement(stmt) = item {
                match self.eval_stmt(stmt) {
                    Ok(v) => result = v,
                    Err(EvalError::Error(e)) => return Err(e),
                    Err(EvalError::Control(_)) => {} 
                }
            }
        }
        Ok(result)
    }
    fn define_function(&mut self, f: &Function) {
        let func = FunctionValue {
            name: f.name.clone(),
            params: f.params.clone(),
            body: f.body.clone(),
            closure: Rc::clone(&self.current),
            is_async: f.is_async,
        };
        self.current.borrow_mut().define(f.name.clone(), Value::Function(Rc::new(func)));
    }
    fn eval_stmt(&mut self, stmt: &Stmt) -> EvalResult {
        match stmt {
            Stmt::Var { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.current.borrow_mut().define(name.clone(), val);
                Ok(Value::Nil)
            }
            Stmt::Const { name, value, .. } => {
                let val = self.eval_expr(value)?;
                self.current.borrow_mut().define(name.clone(), val);
                Ok(Value::Nil)
            }
            Stmt::Assignment { target, value } => {
                let val = self.eval_expr(value)?;
                self.assign_target(target, val)?;
                Ok(Value::Nil)
            }
            Stmt::CompoundAssignment { target, op, value } => {
                let current_val = self.eval_expr(target)?;
                let rhs = self.eval_expr(value)?;
                let new_val = match op {
                    CompoundOp::Add => self.add(&current_val, &rhs)?,
                    CompoundOp::Sub => self.subtract(&current_val, &rhs)?,
                    CompoundOp::Mul => self.multiply(&current_val, &rhs)?,
                    CompoundOp::Div => self.divide(&current_val, &rhs)?,
                };
                self.assign_target(target, new_val)?;
                Ok(Value::Nil)
            }
            Stmt::If { condition, then_block, elif_branches, else_block } => {
                let cond = self.eval_expr(condition)?;
                if cond.is_truthy() {
                    self.eval_block(then_block)
                } else {
                    for (elif_cond, elif_body) in elif_branches {
                        let elif_result = self.eval_expr(elif_cond)?;
                        if elif_result.is_truthy() {
                            return self.eval_block(elif_body);
                        }
                    }
                    if let Some(else_body) = else_block {
                        self.eval_block(else_body)
                    } else {
                        Ok(Value::Nil)
                    }
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    self.iteration_count += 1;
                    if self.iteration_count > MAX_ITERATIONS {
                        return Err(NebulaError::coded(ErrorCode::E071, "while loop").into());
                    }
                    let cond = self.eval_expr(condition)?;
                    if !cond.is_truthy() {
                        break;
                    }
                    match self.eval_block(body) {
                        Ok(_) => {}
                        Err(EvalError::Control(ControlFlow::Break)) => break,
                        Err(EvalError::Control(ControlFlow::Continue)) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(Value::Nil)
            }
            Stmt::For { var, start, end, step, body } => {
                let start_val = self.eval_expr(start)?.as_integer()
                    .ok_or(EvalError::Error(NebulaError::Runtime { message: "for loop start must be integer".to_string() }))?;
                let end_val = self.eval_expr(end)?.as_integer()
                    .ok_or(EvalError::Error(NebulaError::Runtime { message: "for loop end must be integer".to_string() }))?;
                let step_val = if let Some(s) = step {
                    self.eval_expr(s)?.as_integer()
                        .ok_or(EvalError::Error(NebulaError::Runtime { message: "for loop step must be integer".to_string() }))?
                } else {
                    1
                };
                let mut i = start_val;
                while (step_val > 0 && i <= end_val) || (step_val < 0 && i >= end_val) {
                    self.iteration_count += 1;
                    if self.iteration_count > MAX_ITERATIONS {
                        return Err(NebulaError::coded(ErrorCode::E071, "for loop").into());
                    }
                    self.push_scope();
                    self.current.borrow_mut().define(var.clone(), Value::Integer(i));
                    match self.eval_block_inner(body) {
                        Ok(_) => {}
                        Err(EvalError::Control(ControlFlow::Break)) => {
                            self.pop_scope();
                            break;
                        }
                        Err(EvalError::Control(ControlFlow::Continue)) => {
                            self.pop_scope();
                            i += step_val;
                            continue;
                        }
                        Err(e) => {
                            self.pop_scope();
                            return Err(e);
                        }
                    }
                    self.pop_scope();
                    i += step_val;
                }
                Ok(Value::Nil)
            }
            Stmt::Each { var, iterator, body } => {
                let iter_val = self.eval_expr(iterator)?;
                let items: Vec<Value> = match iter_val {
                    Value::Range(start, end, inclusive) => {
                        let end = if inclusive { end + 1 } else { end };
                        (start..end).map(Value::Integer).collect()
                    }
                    Value::List(arr) => arr,
                    Value::String(s) => {
                        s.chars().map(Value::Char).collect()
                    }
                    Value::Map(m) => {
                        m.keys().map(|k| Value::String(k.clone())).collect()
                    }
                    _ => {
                        return Err(NebulaError::InvalidOperation {
                            message: format!("Cannot iterate over {}", iter_val.type_name()),
                        }.into());
                    }
                };
                for item in items {
                    self.push_scope();
                    self.current.borrow_mut().define(var.clone(), item);
                    match self.eval_block_inner(body) {
                        Ok(_) => {}
                        Err(EvalError::Control(ControlFlow::Break)) => {
                            self.pop_scope();
                            break;
                        }
                        Err(EvalError::Control(ControlFlow::Continue)) => {
                            self.pop_scope();
                            continue;
                        }
                        Err(e) => {
                            self.pop_scope();
                            return Err(e);
                        }
                    }
                    self.pop_scope();
                }
                Ok(Value::Nil)
            }
            Stmt::Match { value, arms } => {
                let val = self.eval_expr(value)?;
                for arm in arms {
                    if self.match_pattern(&arm.pattern, &val) {
                        return self.eval_expr(&arm.body);
                    }
                }
                Err(NebulaError::Runtime {
                    message: "Non-exhaustive match".to_string(),
                }.into())
            }
            Stmt::Try { try_block, catch_var, catch_block, finally_block } => {
                let result = self.eval_block(try_block);
                let final_result = match result {
                    Err(EvalError::Error(e)) if catch_block.is_some() => {
                        self.push_scope();
                        if let Some(var) = catch_var {
                            let err_msg = format!("{}", e);
                            self.current.borrow_mut().define(var.clone(), Value::String(err_msg));
                        }
                        let catch_result = self.eval_block_inner(catch_block.as_ref().unwrap());
                        self.pop_scope();
                        catch_result
                    }
                    other => other,
                };
                if let Some(finally) = finally_block {
                    let _ = self.eval_block(finally);
                }
                final_result
            }
            Stmt::Return(expr) => {
                let value = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::Nil
                };
                Err(EvalError::Control(ControlFlow::Return(value)))
            }
            Stmt::Break => Err(EvalError::Control(ControlFlow::Break)),
            Stmt::Continue => Err(EvalError::Control(ControlFlow::Continue)),
            Stmt::Expression(expr) => self.eval_expr(expr),
        }
    }
    fn match_pattern(&self, pattern: &Pattern, value: &Value) -> bool {
        match pattern {
            Pattern::Wildcard => true,
            Pattern::Binding(_) => true,
            Pattern::Literal(lit) => {
                match (lit, value) {
                    (Literal::Integer(a), Value::Integer(b)) => a == b,
                    (Literal::Integer(a), Value::Number(b)) => *a as f64 == *b,
                    (Literal::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
                    (Literal::Float(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
                    (Literal::Bool(a), Value::Bool(b)) => a == b,
                    (Literal::String(a), Value::String(b)) => a == b,
                    _ => false,
                }
            }
        }
    }
    fn assign_target(&mut self, target: &Expr, value: Value) -> EvalResult {
        match target {
            Expr::Variable(name) => {
                if !self.current.borrow_mut().assign(name, value) {
                    return Err(NebulaError::UndefinedVariable { name: name.clone() }.into());
                }
                Ok(Value::Nil)
            }
            Expr::Index { array, index } => {
                if let Expr::Variable(arr_name) = array.as_ref() {
                    let idx = self.eval_expr(index)?.as_integer()
                        .ok_or(EvalError::Error(NebulaError::InvalidOperation { message: "Index must be integer".to_string() }))?;
                    if let Some(Value::List(mut arr)) = self.current.borrow().get(arr_name) {
                        if idx >= 0 && (idx as usize) < arr.len() {
                            arr[idx as usize] = value;
                            self.current.borrow_mut().assign(arr_name, Value::List(arr));
                        } else {
                            return Err(NebulaError::IndexOutOfBounds { index: idx, length: arr.len() }.into());
                        }
                    }
                }
                Ok(Value::Nil)
            }
            Expr::Field { object, field } => {
                if let Expr::Variable(obj_name) = object.as_ref() {
                    if let Some(Value::Map(mut m)) = self.current.borrow().get(obj_name) {
                        m.insert(field.clone(), value);
                        self.current.borrow_mut().assign(obj_name, Value::Map(m));
                    }
                }
                Ok(Value::Nil)
            }
            _ => {
                Err(NebulaError::InvalidOperation {
                    message: "Invalid assignment target".to_string(),
                }.into())
            }
        }
    }
    fn eval_block(&mut self, stmts: &[Stmt]) -> EvalResult {
        self.push_scope();
        let result = self.eval_block_inner(stmts);
        self.pop_scope();
        result
    }
    fn eval_block_inner(&mut self, stmts: &[Stmt]) -> EvalResult {
        let mut result = Value::Nil;
        for stmt in stmts {
            result = self.eval_stmt(stmt)?;
        }
        Ok(result)
    }
    fn eval_expr(&mut self, expr: &Expr) -> EvalResult {
        match expr {
            Expr::Literal(lit) => Ok(self.eval_literal(lit)),
            Expr::Variable(name) => {
                self.current.borrow().get(name)
                    .ok_or_else(|| NebulaError::UndefinedVariable { name: name.clone() }.into())
            }
            Expr::Binary { left, op, right } => {
                let lhs = self.eval_expr(left)?;
                let rhs = self.eval_expr(right)?;
                self.eval_binary_op(*op, &lhs, &rhs)
            }
            Expr::Unary { op, operand } => {
                let val = self.eval_expr(operand)?;
                self.eval_unary_op(*op, &val)
            }
            Expr::Call { callee, args } => {
                let callee_val = self.eval_expr(callee)?;
                let arg_vals: Result<Vec<_>, _> = args.iter()
                    .map(|a| self.eval_expr(a))
                    .collect();
                let arg_vals = arg_vals?;
                match callee_val {
                    Value::Function(func) => self.call_function(&func, &arg_vals),
                    Value::Lambda(lambda) => self.call_lambda(&lambda, &arg_vals),
                    Value::NativeFunction(nf) => {
                        if let Some(arity) = nf.arity {
                            if arg_vals.len() != arity {
                                return Err(NebulaError::InvalidOperation {
                                    message: format!("{}() expected {} arguments, got {}", nf.name, arity, arg_vals.len()),
                                }.into());
                            }
                        }
                        (nf.func)(&arg_vals).map_err(|msg| NebulaError::Runtime { message: msg }.into())
                    }
                    _ => Err(NebulaError::InvalidOperation {
                        message: format!("Cannot call {}", callee_val.type_name()),
                    }.into()),
                }
            }
            Expr::MethodCall { receiver, method, args } => {
                let recv_val = self.eval_expr(receiver)?;
                let arg_vals: Result<Vec<_>, _> = args.iter()
                    .map(|a| self.eval_expr(a))
                    .collect();
                let arg_vals = arg_vals?;
                self.call_method(&recv_val, method, &arg_vals)
            }
            Expr::Field { object, field } => {
                let obj = self.eval_expr(object)?;
                self.get_field(&obj, field)
            }
            Expr::Index { array, index } => {
                let arr = self.eval_expr(array)?;
                let idx = self.eval_expr(index)?;
                self.get_index(&arr, &idx)
            }
            Expr::Slice { array, start, end } => {
                let arr = self.eval_expr(array)?;
                let start_idx = start.as_ref().map(|e| self.eval_expr(e)).transpose()?
                    .and_then(|v| v.as_integer());
                let end_idx = end.as_ref().map(|e| self.eval_expr(e)).transpose()?
                    .and_then(|v| v.as_integer());
                match arr {
                    Value::List(list) => {
                        let s = start_idx.unwrap_or(0).max(0) as usize;
                        let e = end_idx.map(|i| i as usize).unwrap_or(list.len()).min(list.len());
                        Ok(Value::List(list[s..e].to_vec()))
                    }
                    Value::String(string) => {
                        let chars: Vec<_> = string.chars().collect();
                        let s = start_idx.unwrap_or(0).max(0) as usize;
                        let e = end_idx.map(|i| i as usize).unwrap_or(chars.len()).min(chars.len());
                        Ok(Value::String(chars[s..e].iter().collect()))
                    }
                    _ => Err(NebulaError::InvalidOperation {
                        message: format!("Cannot slice {}", arr.type_name()),
                    }.into()),
                }
            }
            Expr::Ternary { condition, then_expr, else_expr } => {
                let cond = self.eval_expr(condition)?;
                if cond.is_truthy() {
                    self.eval_expr(then_expr)
                } else {
                    self.eval_expr(else_expr)
                }
            }
            Expr::Lambda { params, body } => {
                let lambda = LambdaValue {
                    params: params.clone(),
                    body: (**body).clone(),
                    closure: Rc::clone(&self.current),
                };
                Ok(Value::Lambda(Rc::new(lambda)))
            }
            Expr::List(elements) => {
                let vals: Result<Vec<_>, _> = elements.iter()
                    .map(|e| self.eval_expr(e))
                    .collect();
                Ok(Value::List(vals?))
            }
            Expr::Map(pairs) => {
                let mut map = HashMap::new();
                for (key, value) in pairs {
                    let k = match self.eval_expr(key)? {
                        Value::String(s) => s,
                        other => other.to_display_string(),
                    };
                    let v = self.eval_expr(value)?;
                    map.insert(k, v);
                }
                Ok(Value::Map(map))
            }
            Expr::Tuple(elements) => {
                let vals: Result<Vec<_>, _> = elements.iter()
                    .map(|e| self.eval_expr(e))
                    .collect();
                Ok(Value::Tuple(vals?))
            }
            Expr::Range { start, end, inclusive } => {
                let s = self.eval_expr(start)?.as_integer()
                    .ok_or(EvalError::Error(NebulaError::InvalidOperation { message: "Range start must be integer".to_string() }))?;
                let e = self.eval_expr(end)?.as_integer()
                    .ok_or(EvalError::Error(NebulaError::InvalidOperation { message: "Range end must be integer".to_string() }))?;
                Ok(Value::Range(s, e, *inclusive))
            }
            Expr::StructInit { name, args } => {
                let arg_vals: Result<Vec<_>, _> = args.iter()
                    .map(|e| self.eval_expr(e))
                    .collect();
                Ok(Value::Struct {
                    name: name.clone(),
                    fields: arg_vals?,
                })
            }
            Expr::Length(operand) => {
                let val = self.eval_expr(operand)?;
                match val {
                    Value::List(arr) => Ok(Value::Integer(arr.len() as i64)),
                    Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                    Value::Map(m) => Ok(Value::Integer(m.len() as i64)),
                    _ => Err(NebulaError::InvalidOperation {
                        message: format!("Cannot get length of {}", val.type_name()),
                    }.into()),
                }
            }
            Expr::Append { list, value } => {
                let mut arr = match self.eval_expr(list)? {
                    Value::List(a) => a,
                    other => return Err(NebulaError::InvalidOperation {
                        message: format!("Cannot append to {}", other.type_name()),
                    }.into()),
                };
                let val = self.eval_expr(value)?;
                arr.push(val);
                Ok(Value::List(arr))
            }
            Expr::Await(operand) => self.eval_expr(operand),
            Expr::Spawn(operand) => self.eval_expr(operand),
            Expr::Error(msg) => {
                let message = self.eval_expr(msg)?.to_display_string();
                Err(NebulaError::Runtime { message }.into())
            }
            Expr::Assert { condition, message } => {
                let cond = self.eval_expr(condition)?;
                if !cond.is_truthy() {
                    let msg = if let Some(m) = message {
                        self.eval_expr(m)?.to_display_string()
                    } else {
                        "Assertion failed".to_string()
                    };
                    return Err(NebulaError::Runtime { message: msg }.into());
                }
                Ok(Value::Nil)
            }
            Expr::Send { channel, value } => {
                if let Value::Channel(ch) = self.eval_expr(channel)? {
                    let val = self.eval_expr(value)?;
                    ch.borrow_mut().push(val);
                    Ok(Value::Nil)
                } else {
                    Err(NebulaError::InvalidOperation { message: "Send requires channel".to_string() }.into())
                }
            }
            Expr::Receive(channel) => {
                if let Value::Channel(ch) = self.eval_expr(channel)? {
                    ch.borrow_mut().pop().ok_or(NebulaError::Runtime {
                        message: "Channel empty".to_string()
                    }.into())
                } else {
                    Err(NebulaError::InvalidOperation { message: "Receive requires channel".to_string() }.into())
                }
            }
            Expr::Borrow(operand) => self.eval_expr(operand),
            Expr::Cast { ty, value } => {
                let val = self.eval_expr(value)?;
                self.cast_value(ty, val)
            }
            Expr::TypeOf(operand) => {
                let val = self.eval_expr(operand)?;
                Ok(Value::String(val.type_name().to_string()))
            }
            Expr::Block(stmts) => self.eval_block(stmts),
            Expr::Nil => Ok(Value::Nil),
        }
    }
    fn eval_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Integer(n) => Value::Number(*n as f64),
            Literal::Float(f) => Value::Number(*f),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
        }
    }
    fn eval_binary_op(&self, op: BinaryOp, lhs: &Value, rhs: &Value) -> EvalResult {
        match op {
            BinaryOp::Add => self.add(lhs, rhs),
            BinaryOp::Sub => self.subtract(lhs, rhs),
            BinaryOp::Mul => self.multiply(lhs, rhs),
            BinaryOp::Div => self.divide(lhs, rhs),
            BinaryOp::Mod => self.modulo(lhs, rhs),
            BinaryOp::Pow => self.power(lhs, rhs),
            BinaryOp::Eq => Ok(Value::Bool(lhs == rhs)),
            BinaryOp::Ne => Ok(Value::Bool(lhs != rhs)),
            BinaryOp::Lt => self.compare_lt(lhs, rhs),
            BinaryOp::Gt => self.compare_gt(lhs, rhs),
            BinaryOp::Le => self.compare_le(lhs, rhs),
            BinaryOp::Ge => self.compare_ge(lhs, rhs),
            BinaryOp::And => Ok(Value::Bool(lhs.is_truthy() && rhs.is_truthy())),
            BinaryOp::Or => Ok(Value::Bool(lhs.is_truthy() || rhs.is_truthy())),
            BinaryOp::BitAnd => self.bitand(lhs, rhs),
            BinaryOp::BitOr => self.bitor(lhs, rhs),
            BinaryOp::BitXor => self.bitxor(lhs, rhs),
            BinaryOp::Shl => self.shl(lhs, rhs),
            BinaryOp::Shr => self.shr(lhs, rhs),
        }
    }
    fn add(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), other) => Ok(Value::String(format!("{}{}", a, other))),
            (other, Value::String(b)) => Ok(Value::String(format!("{}{}", other, b))),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot add {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn subtract(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Number(a - (*b as f64))),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Number((*a as f64) - b)),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot subtract {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn multiply(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Number(a * (*b as f64))),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Number((*a as f64) * b)),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot multiply {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn divide(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 { Err(NebulaError::DivisionByZero.into()) }
                else { Ok(Value::Number(a / b)) }
            }
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 { Err(NebulaError::DivisionByZero.into()) }
                else { Ok(Value::Integer(a / b)) }
            }
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot divide {} by {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn modulo(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 { Err(NebulaError::DivisionByZero.into()) }
                else { Ok(Value::Integer(a % b)) }
            }
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Number(a % (*b as f64))),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Number((*a as f64) % b)),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot modulo {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn power(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        let base = lhs.as_number().ok_or(EvalError::Error(NebulaError::InvalidOperation {
            message: "Power requires numbers".to_string(),
        }))?;
        let exp = rhs.as_number().ok_or(EvalError::Error(NebulaError::InvalidOperation {
            message: "Power requires numbers".to_string(),
        }))?;
        Ok(Value::Number(base.powf(exp)))
    }
    fn compare_lt(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a < b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a < b)),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot compare {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn compare_gt(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a > b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a > b)),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot compare {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn compare_le(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a <= b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a <= b)),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot compare {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn compare_ge(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a >= b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a >= b)),
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot compare {} and {}", lhs.type_name(), rhs.type_name()),
            }.into()),
        }
    }
    fn bitand(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a & b)),
            _ => Err(NebulaError::InvalidOperation { message: "Bitwise AND requires integers".to_string() }.into()),
        }
    }
    fn bitor(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a | b)),
            _ => Err(NebulaError::InvalidOperation { message: "Bitwise OR requires integers".to_string() }.into()),
        }
    }
    fn bitxor(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a ^ b)),
            _ => Err(NebulaError::InvalidOperation { message: "Bitwise XOR requires integers".to_string() }.into()),
        }
    }
    fn shl(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a << b)),
            _ => Err(NebulaError::InvalidOperation { message: "Shift requires integers".to_string() }.into()),
        }
    }
    fn shr(&self, lhs: &Value, rhs: &Value) -> EvalResult {
        match (lhs, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a >> b)),
            _ => Err(NebulaError::InvalidOperation { message: "Shift requires integers".to_string() }.into()),
        }
    }
    fn eval_unary_op(&self, op: UnaryOp, val: &Value) -> EvalResult {
        match op {
            UnaryOp::Neg => {
                match val {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    Value::Integer(n) => Ok(Value::Integer(-n)),
                    _ => Err(NebulaError::InvalidOperation {
                        message: format!("Cannot negate {}", val.type_name()),
                    }.into()),
                }
            }
            UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
            UnaryOp::BitNot => {
                match val {
                    Value::Integer(n) => Ok(Value::Integer(!n)),
                    _ => Err(NebulaError::InvalidOperation {
                        message: format!("Cannot bitwise NOT {}", val.type_name()),
                    }.into()),
                }
            }
        }
    }
    fn call_function(&mut self, func: &FunctionValue, args: &[Value]) -> EvalResult {
        self.recursion_depth += 1;
        if self.recursion_depth > MAX_RECURSION_DEPTH {
            self.recursion_depth -= 1;
            return Err(NebulaError::Runtime {
                message: format!("Maximum recursion depth ({}) exceeded", MAX_RECURSION_DEPTH),
            }.into());
        }
        let prev = Rc::clone(&self.current);
        let new_env = Environment::with_parent(Rc::clone(&func.closure));
        self.current = Rc::new(RefCell::new(new_env));
        for (i, param) in func.params.iter().enumerate() {
            let value = if i < args.len() {
                args[i].clone()
            } else if let Some(default) = &param.default {
                self.eval_expr(default)?
            } else if param.variadic {
                Value::List(args[i..].to_vec())
            } else {
                Value::Nil
            };
            self.current.borrow_mut().define(param.name.clone(), value);
        }
        let result = match &func.body {
            FunctionBody::Expression(expr) => self.eval_expr(expr),
            FunctionBody::Block(stmts) => {
                let mut res = Value::Nil;
                for stmt in stmts {
                    match self.eval_stmt(stmt) {
                        Ok(v) => res = v,
                        Err(EvalError::Control(ControlFlow::Return(value))) => {
                            self.current = prev;
                            return Ok(value);
                        }
                        Err(e) => {
                            self.current = prev;
                            return Err(e);
                        }
                    }
                }
                Ok(res)
            }
        };
        self.current = prev;
        self.recursion_depth -= 1;
        result
    }
    fn call_lambda(&mut self, lambda: &LambdaValue, args: &[Value]) -> EvalResult {
        self.recursion_depth += 1;
        if self.recursion_depth > MAX_RECURSION_DEPTH {
            self.recursion_depth -= 1;
            return Err(NebulaError::Runtime {
                message: format!("Maximum recursion depth ({}) exceeded", MAX_RECURSION_DEPTH),
            }.into());
        }
        let prev = Rc::clone(&self.current);
        let new_env = Environment::with_parent(Rc::clone(&lambda.closure));
        self.current = Rc::new(RefCell::new(new_env));
        for (i, param) in lambda.params.iter().enumerate() {
            let value = args.get(i).cloned().unwrap_or(Value::Nil);
            self.current.borrow_mut().define(param.clone(), value);
        }
        let result = self.eval_expr(&lambda.body);
        self.current = prev;
        self.recursion_depth -= 1;
        result
    }
    fn call_method(&mut self, receiver: &Value, method: &str, args: &[Value]) -> EvalResult {
        match (receiver, method) {
            (Value::List(arr), "len") => Ok(Value::Integer(arr.len() as i64)),
            (Value::List(arr), "push") if !args.is_empty() => {
                let mut new_arr = arr.clone();
                for arg in args {
                    new_arr.push(arg.clone());
                }
                Ok(Value::List(new_arr))
            }
            (Value::List(arr), "pop") => {
                let mut new_arr = arr.clone();
                let val = new_arr.pop().unwrap_or(Value::Nil);
                Ok(val)
            }
            (Value::String(s), "len") => Ok(Value::Integer(s.len() as i64)),
            (Value::String(s), "upper") => Ok(Value::String(s.to_uppercase())),
            (Value::String(s), "lower") => Ok(Value::String(s.to_lowercase())),
            (Value::String(s), "trim") => Ok(Value::String(s.trim().to_string())),
            (Value::String(s), "split") if !args.is_empty() => {
                let sep = args[0].to_display_string();
                let parts: Vec<_> = s.split(&sep).map(|p| Value::String(p.to_string())).collect();
                Ok(Value::List(parts))
            }
            (Value::Map(m), "keys") => {
                Ok(Value::List(m.keys().map(|k| Value::String(k.clone())).collect()))
            }
            (Value::Map(m), "values") => {
                Ok(Value::List(m.values().cloned().collect()))
            }
            _ => Err(NebulaError::Runtime {
                message: format!("No method '{}' on {}", method, receiver.type_name()),
            }.into()),
        }
    }
    fn get_field(&self, obj: &Value, field: &str) -> EvalResult {
        match obj {
            Value::Map(m) => {
                m.get(field).cloned().ok_or_else(|| NebulaError::Runtime {
                    message: format!("Key '{}' not found", field),
                }.into())
            }
            Value::Struct { name, fields } => {
                if let Some(field_names) = self.structs.get(name) {
                    if let Some(idx) = field_names.iter().position(|n| n == field) {
                        return fields.get(idx).cloned().ok_or_else(|| NebulaError::Runtime {
                            message: format!("Field '{}' not found", field),
                        }.into());
                    }
                }
                Err(NebulaError::Runtime {
                    message: format!("Field '{}' not found on {}", field, name),
                }.into())
            }
            Value::Tuple(elements) => {
                if let Ok(idx) = field.parse::<usize>() {
                    elements.get(idx).cloned().ok_or_else(|| NebulaError::IndexOutOfBounds {
                        index: idx as i64,
                        length: elements.len(),
                    }.into())
                } else {
                    Err(NebulaError::Runtime {
                        message: format!("Invalid tuple index: {}", field),
                    }.into())
                }
            }
            _ => Err(NebulaError::Runtime {
                message: format!("Cannot access field on {}", obj.type_name()),
            }.into()),
        }
    }
    fn get_index(&self, arr: &Value, idx: &Value) -> EvalResult {
        match (arr, idx) {
            (Value::List(list), idx) => {
                let i = idx.as_integer().ok_or(EvalError::Error(NebulaError::InvalidOperation {
                    message: "Index must be integer".to_string(),
                }))?;
                if i < 0 || i as usize >= list.len() {
                    Err(NebulaError::IndexOutOfBounds { index: i, length: list.len() }.into())
                } else {
                    Ok(list[i as usize].clone())
                }
            }
            (Value::String(s), idx) => {
                let i = idx.as_integer().ok_or(EvalError::Error(NebulaError::InvalidOperation {
                    message: "Index must be integer".to_string(),
                }))?;
                let chars: Vec<_> = s.chars().collect();
                if i < 0 || i as usize >= chars.len() {
                    Err(NebulaError::IndexOutOfBounds { index: i, length: chars.len() }.into())
                } else {
                    Ok(Value::Char(chars[i as usize]))
                }
            }
            (Value::Map(m), idx) => {
                let key = idx.to_display_string();
                m.get(&key).cloned().ok_or_else(|| NebulaError::Runtime {
                    message: format!("Key '{}' not found", key),
                }.into())
            }
            _ => Err(NebulaError::InvalidOperation {
                message: format!("Cannot index {} with {}", arr.type_name(), idx.type_name()),
            }.into()),
        }
    }
    fn cast_value(&self, ty: &Type, val: Value) -> EvalResult {
        match ty {
            Type::Nb => {
                let n = val.as_number().ok_or(EvalError::Error(NebulaError::InvalidOperation {
                    message: "Cannot convert to number".to_string(),
                }))?;
                Ok(Value::Number(n))
            }
            Type::Int => {
                let n = val.as_integer().ok_or(EvalError::Error(NebulaError::InvalidOperation {
                    message: "Cannot convert to integer".to_string(),
                }))?;
                Ok(Value::Integer(n))
            }
            Type::Wrd => {
                Ok(Value::String(val.to_display_string()))
            }
            _ => Ok(val),
        }
    }
    fn push_scope(&mut self) {
        let new_env = Environment::with_parent(Rc::clone(&self.current));
        self.current = Rc::new(RefCell::new(new_env));
    }
    fn pop_scope(&mut self) {
        let parent = self.current.borrow().parent();
        if let Some(p) = parent {
            self.current = p;
        }
    }
}
impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
