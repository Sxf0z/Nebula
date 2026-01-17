use crate::error::{SpectreError, SpectreResult, ErrorCode};
use super::{Chunk, OpCode, NanBoxed, HeapObject, CompiledFunction, HeapData};
const STACK_SIZE: usize = 256;
const MAX_GLOBALS: usize = 256;
const MAX_FRAMES: usize = 64;
const MAX_ITERATIONS: usize = 1_000_000;
const BUILTIN_COUNT: usize = 21;
pub const BUILTIN_NAMES: [&str; BUILTIN_COUNT] = [
    "log", "typeof", "sqrt", "abs", "len", "floor", "ceil", 
    "round", "pow", "sin", "cos", "tan", "exp", "ln", "get", 
    "rnd", "dbg", "now", "sleep", "str", "num"
];
#[derive(Clone)]
struct CallFrame {
    function: Option<*mut HeapObject>,
    ip: usize,
    base: usize,
}
pub struct VMNanBox {
    stack: Vec<NanBoxed>,
    frames: Vec<CallFrame>,
    ip: usize,
    frame_base: usize,
    globals: Vec<NanBoxed>,
    global_names: Vec<String>,
    iteration_count: usize,
}
impl VMNanBox {
    pub fn new() -> Self {
        let mut globals = vec![NanBoxed::nil(); MAX_GLOBALS];
        for (i, name) in BUILTIN_NAMES.iter().enumerate() {
            let ptr = HeapObject::new_string(name);
            globals[i] = NanBoxed::ptr(ptr);
        }
        Self {
            stack: Vec::with_capacity(STACK_SIZE),
            frames: Vec::with_capacity(MAX_FRAMES),
            ip: 0,
            frame_base: 0,
            globals,
            global_names: Vec::new(),
            iteration_count: 0,
        }
    }
    pub fn run(&mut self, chunk: &Chunk, global_names: &[String]) -> SpectreResult<NanBoxed> {
        self.run_with_functions(chunk, global_names, &[])
    }
    pub fn run_with_functions(
        &mut self, 
        chunk: &Chunk, 
        global_names: &[String],
        functions: &[CompiledFunction]
    ) -> SpectreResult<NanBoxed> {
        self.ip = 0;
        self.frame_base = 0;
        self.iteration_count = 0;
        self.global_names = global_names.to_vec();
        self.frames.clear();
        self.stack.clear();
        self.frames.push(CallFrame {
            function: None,
            ip: 0,
            base: 0,
        });
        self.run_main_loop(chunk, functions)
    }
    fn run_main_loop(&mut self, chunk: &Chunk, functions: &[CompiledFunction]) -> SpectreResult<NanBoxed> {
        loop {
            if self.ip >= chunk.code().len() {
                break;
            }
            let byte = chunk.read_byte(self.ip);
            let op = match OpCode::from_byte(byte) {
                Some(op) => op,
                None => return Err(SpectreError::coded(ErrorCode::E004, format!("invalid opcode {}", byte))),
            };
            self.ip += 1;
            match op {
                OpCode::PushConst => {
                    let idx = chunk.read_byte(self.ip);
                    self.ip += 1;
                    let value = chunk.get_constant(idx);
                    let nb = self.value_to_nanbox(value);
                    self.push(nb)?;
                }
                OpCode::PushNil => self.push(NanBoxed::nil())?,
                OpCode::PushTrue => self.push(NanBoxed::boolean(true))?,
                OpCode::PushFalse => self.push(NanBoxed::boolean(false))?,
                OpCode::Pop => { self.pop()?; }
                OpCode::Dup => {
                    let value = self.peek(0)?;
                    self.push(value)?;
                }
                OpCode::LoadLocal => {
                    let slot = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    let value = self.stack[slot];
                    self.push(value)?;
                }
                OpCode::StoreLocal => {
                    let slot = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    let value = self.peek(0)?;
                    self.stack[slot] = value;
                }
                OpCode::LoadGlobal => {
                    let idx = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    if idx >= self.globals.len() {
                        return Err(SpectreError::coded(ErrorCode::E013, format!("global index {} out of bounds", idx)));
                    }
                    let value = self.globals[idx];
                    self.push(value)?;
                }
                OpCode::StoreGlobal => {
                    let idx = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    if idx >= self.globals.len() {
                        return Err(SpectreError::coded(ErrorCode::E013, format!("global index {} out of bounds", idx)));
                    }
                    let value = self.peek(0)?;
                    self.globals[idx] = value;
                }
                OpCode::DefineGlobal => {
                    let idx = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    if idx >= self.globals.len() {
                        return Err(SpectreError::coded(ErrorCode::E013, format!("global index {} out of bounds", idx)));
                    }
                    let value = self.pop()?;
                    self.globals[idx] = value;
                }
                OpCode::LoadLocal0 => {
                    let value = self.stack[0];
                    self.push(value)?;
                }
                OpCode::LoadLocal1 => {
                    let value = self.stack[1];
                    self.push(value)?;
                }
                OpCode::LoadLocal2 => {
                    let value = self.stack[2];
                    self.push(value)?;
                }
                OpCode::StoreLocal0 => {
                    let value = self.peek(0)?;
                    self.stack[0] = value;
                }
                OpCode::StoreLocal1 => {
                    let value = self.peek(0)?;
                    self.stack[1] = value;
                }
                OpCode::StoreLocal2 => {
                    let value = self.peek(0)?;
                    self.stack[2] = value;
                }
                OpCode::LoadGlobal0 => {
                    let value = self.globals[21];
                    self.push(value)?;
                }
                OpCode::LoadGlobal1 => {
                    let value = self.globals[22];
                    self.push(value)?;
                }
                OpCode::LoadGlobal2 => {
                    let value = self.globals[23];
                    self.push(value)?;
                }
                OpCode::StoreGlobal0 => {
                    let value = self.peek(0)?;
                    self.globals[21] = value;
                }
                OpCode::StoreGlobal1 => {
                    let value = self.peek(0)?;
                    self.globals[22] = value;
                }
                OpCode::StoreGlobal2 => {
                    let value = self.peek(0)?;
                    self.globals[23] = value;
                }
                OpCode::AddInt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(NanBoxed::integer(a.as_integer() + b.as_integer()))?;
                }
                OpCode::SubInt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(NanBoxed::integer(a.as_integer() - b.as_integer()))?;
                }
                OpCode::MulInt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(NanBoxed::integer(a.as_integer() * b.as_integer()))?;
                }
                OpCode::IncLocal => {
                    let slot = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    let value = self.stack[slot];
                    if value.is_integer() {
                        self.stack[slot] = NanBoxed::integer(value.as_integer() + 1);
                    } else if value.is_number() {
                        self.stack[slot] = NanBoxed::number(value.as_number() + 1.0);
                    }
                }
                OpCode::DecLocal => {
                    let slot = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    let value = self.stack[slot];
                    if value.is_integer() {
                        self.stack[slot] = NanBoxed::integer(value.as_integer() - 1);
                    } else if value.is_number() {
                        self.stack[slot] = NanBoxed::number(value.as_number() - 1.0);
                    }
                }
                OpCode::Inc => {
                    let v = self.pop()?;
                    if v.is_integer() {
                        self.push(NanBoxed::integer(v.as_integer() + 1))?;
                    } else if v.is_number() {
                        self.push(NanBoxed::number(v.as_number() + 1.0))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "inc"));
                    }
                }
                OpCode::Dec => {
                    let v = self.pop()?;
                    if v.is_integer() {
                        self.push(NanBoxed::integer(v.as_integer() - 1))?;
                    } else if v.is_number() {
                        self.push(NanBoxed::number(v.as_number() - 1.0))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "dec"));
                    }
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if a.is_number() && b.is_number() {
                        self.push(NanBoxed::number(a.as_number() + b.as_number()))?;
                    }
                    else if a.is_integer() && b.is_integer() {
                        self.push(NanBoxed::integer(a.as_integer() + b.as_integer()))?;
                    }
                    else if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(na + nb))?;
                    }
                    else {
                        return Err(SpectreError::coded(ErrorCode::E031, "add"));
                    }
                }
                OpCode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if a.is_number() && b.is_number() {
                        self.push(NanBoxed::number(a.as_number() - b.as_number()))?;
                    } else if a.is_integer() && b.is_integer() {
                        self.push(NanBoxed::integer(a.as_integer() - b.as_integer()))?;
                    } else if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(na - nb))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "sub"));
                    }
                }
                OpCode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if a.is_number() && b.is_number() {
                        self.push(NanBoxed::number(a.as_number() * b.as_number()))?;
                    } else if a.is_integer() && b.is_integer() {
                        self.push(NanBoxed::integer(a.as_integer() * b.as_integer()))?;
                    } else if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(na * nb))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "mul"));
                    }
                }
                OpCode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let nb = b.as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "div"))?;
                    let na = a.as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "div"))?;
                    if nb == 0.0 {
                        return Err(SpectreError::coded(ErrorCode::E040, ""));
                    }
                    self.push(NanBoxed::number(na / nb))?;
                }
                OpCode::Mod => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(na % nb))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "mod"));
                    }
                }
                OpCode::Pow => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(na.powf(nb)))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "pow"));
                    }
                }
                OpCode::Neg => {
                    let v = self.pop()?;
                    if v.is_number() {
                        self.push(NanBoxed::number(-v.as_number()))?;
                    } else if v.is_integer() {
                        self.push(NanBoxed::integer(-v.as_integer()))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "neg"));
                    }
                }
                OpCode::Eq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(NanBoxed::boolean(self.values_equal(a, b)))?;
                }
                OpCode::Ne => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(NanBoxed::boolean(!self.values_equal(a, b)))?;
                }
                OpCode::Lt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::boolean(na < nb))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "lt"));
                    }
                }
                OpCode::Gt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::boolean(na > nb))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "gt"));
                    }
                }
                OpCode::Le => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::boolean(na <= nb))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "le"));
                    }
                }
                OpCode::Ge => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::boolean(na >= nb))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "ge"));
                    }
                }
                OpCode::Not => {
                    let v = self.pop()?;
                    self.push(NanBoxed::boolean(!v.is_truthy()))?;
                }
                OpCode::And => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    if !self.peek(0)?.is_truthy() {
                        self.ip += offset;
                    } else {
                        self.pop()?;
                    }
                }
                OpCode::Or => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    if self.peek(0)?.is_truthy() {
                        self.ip += offset;
                    } else {
                        self.pop()?;
                    }
                }
                OpCode::Jump => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    self.ip += offset;
                }
                OpCode::JumpIfFalse => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    if !self.peek(0)?.is_truthy() {
                        self.ip += offset;
                    }
                }
                OpCode::JumpIfTrue => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    if self.peek(0)?.is_truthy() {
                        self.ip += offset;
                    }
                }
                OpCode::Loop => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    self.ip -= offset;
                }
                OpCode::Return => {
                    let result = if self.stack.is_empty() {
                        NanBoxed::nil()
                    } else {
                        self.pop()?
                    };
                    return Ok(result);
                }
                OpCode::CheckIterLimit => {
                    self.iteration_count += 1;
                    if self.iteration_count > MAX_ITERATIONS {
                        return Err(SpectreError::coded(ErrorCode::E071, "vm loop"));
                    }
                }
                OpCode::Call => {
                    let argc = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    let callee = self.peek(argc)?;
                    if callee.is_ptr() {
                        debug_assert!(!callee.as_ptr().is_null(), "null pointer in Call");
                        let obj = unsafe { &*callee.as_ptr() };
                        match &obj.data {
                            super::HeapData::String(name) => {
                                let result = self.call_builtin(name, argc)?;
                                for _ in 0..=argc {
                                    self.pop()?;
                                }
                                self.push(result)?;
                            }
                            super::HeapData::Function(func) => {
                                if argc != func.arity as usize {
                                    return Err(SpectreError::coded(
                                        ErrorCode::E012, 
                                        format!("{}: expected {} args, got {}", func.name, func.arity, argc)
                                    ));
                                }
                                if self.frames.len() >= MAX_FRAMES {
                                    return Err(SpectreError::coded(
                                        ErrorCode::E071, 
                                        format!("stack overflow: max {} frames", MAX_FRAMES)
                                    ));
                                }
                                let base = self.stack.len() - argc;
                                let saved_ip = self.ip;
                                let saved_frame_base = self.frame_base;
                                self.ip = 0;
                                self.frame_base = base;
                                let func_chunk = &func.chunk;
                                let result = self.execute_function_body(func_chunk)?;
                                self.ip = saved_ip;
                                self.frame_base = saved_frame_base;
                                for _ in 0..=argc {
                                    self.pop()?;
                                }
                                self.push(result)?;
                            }
                            _ => {
                                return Err(SpectreError::coded(ErrorCode::E011, "not callable"));
                            }
                        }
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E011, "not callable"));
                    }
                }
                OpCode::List => {
                    let count = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    let mut items = Vec::with_capacity(count);
                    for _ in 0..count {
                        items.push(self.pop()?);
                    }
                    items.reverse();
                    let ptr = HeapObject::new_list(items);
                    self.push(NanBoxed::ptr(ptr))?;
                }
                OpCode::Closure => {
                    let func_idx = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    if func_idx < functions.len() {
                        let func = functions[func_idx].clone();
                        let ptr = HeapObject::new_function(func);
                        self.push(NanBoxed::ptr(ptr))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E004, format!("invalid function index {}", func_idx)));
                    }
                }
                _ => {
                    return Err(SpectreError::coded(ErrorCode::E004, format!("unhandled opcode {:?}", op)));
                }
            }
        }
        Ok(if self.stack.is_empty() {
            NanBoxed::nil()
        } else {
            self.pop()?
        })
    }
    fn execute_function_body(&mut self, chunk: &Chunk) -> SpectreResult<NanBoxed> {
        loop {
            if self.ip >= chunk.code().len() {
                break;
            }
            let byte = chunk.read_byte(self.ip);
            let op = match OpCode::from_byte(byte) {
                Some(op) => op,
                None => return Err(SpectreError::coded(ErrorCode::E004, format!("invalid opcode {}", byte))),
            };
            self.ip += 1;
            match op {
                OpCode::Return => {
                    return Ok(if self.stack.len() > self.frame_base {
                        self.pop()?
                    } else {
                        NanBoxed::nil()
                    });
                }
                OpCode::PushConst => {
                    let idx = chunk.read_byte(self.ip);
                    self.ip += 1;
                    let value = chunk.get_constant(idx);
                    let nb = self.value_to_nanbox(value);
                    self.push(nb)?;
                }
                OpCode::PushNil => self.push(NanBoxed::nil())?,
                OpCode::PushTrue => self.push(NanBoxed::boolean(true))?,
                OpCode::PushFalse => self.push(NanBoxed::boolean(false))?,
                OpCode::Pop => { self.pop()?; }
                OpCode::LoadLocal | OpCode::LoadLocal0 | OpCode::LoadLocal1 | OpCode::LoadLocal2 => {
                    let slot = match op {
                        OpCode::LoadLocal => {
                            let s = chunk.read_byte(self.ip) as usize;
                            self.ip += 1;
                            s
                        }
                        OpCode::LoadLocal0 => 0,
                        OpCode::LoadLocal1 => 1,
                        OpCode::LoadLocal2 => 2,
                        _ => unreachable!()
                    };
                    let value = self.stack[self.frame_base + slot];
                    self.push(value)?;
                }
                OpCode::StoreLocal | OpCode::StoreLocal0 | OpCode::StoreLocal1 | OpCode::StoreLocal2 => {
                    let slot = match op {
                        OpCode::StoreLocal => {
                            let s = chunk.read_byte(self.ip) as usize;
                            self.ip += 1;
                            s
                        }
                        OpCode::StoreLocal0 => 0,
                        OpCode::StoreLocal1 => 1,
                        OpCode::StoreLocal2 => 2,
                        _ => unreachable!()
                    };
                    let value = self.peek(0)?;
                    self.stack[self.frame_base + slot] = value;
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(av), Some(bv)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(av + bv))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "add"));
                    }
                }
                OpCode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(av), Some(bv)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(av - bv))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "sub"));
                    }
                }
                OpCode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(av), Some(bv)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::number(av * bv))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "mul"));
                    }
                }
                OpCode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(av), Some(bv)) = (a.as_numeric(), b.as_numeric()) {
                        if bv == 0.0 {
                            return Err(SpectreError::coded(ErrorCode::E040, ""));
                        }
                        self.push(NanBoxed::number(av / bv))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "div"));
                    }
                }
                OpCode::Neg => {
                    let v = self.pop()?;
                    if let Some(n) = v.as_numeric() {
                        self.push(NanBoxed::number(-n))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "neg"));
                    }
                }
                OpCode::Eq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(NanBoxed::boolean(self.values_equal(a, b)))?;
                }
                OpCode::Ne => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(NanBoxed::boolean(!self.values_equal(a, b)))?;
                }
                OpCode::Lt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(av), Some(bv)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::boolean(av < bv))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "lt"));
                    }
                }
                OpCode::Gt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if let (Some(av), Some(bv)) = (a.as_numeric(), b.as_numeric()) {
                        self.push(NanBoxed::boolean(av > bv))?;
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E031, "gt"));
                    }
                }
                OpCode::LoadGlobal => {
                    let idx = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    if idx >= self.globals.len() {
                        return Err(SpectreError::coded(ErrorCode::E013, format!("global index {} out of bounds", idx)));
                    }
                    let value = self.globals[idx];
                    self.push(value)?;
                }
                OpCode::LoadGlobal0 => {
                    let value = self.globals[21];
                    self.push(value)?;
                }
                OpCode::LoadGlobal1 => {
                    let value = self.globals[22];
                    self.push(value)?;
                }
                OpCode::LoadGlobal2 => {
                    let value = self.globals[23];
                    self.push(value)?;
                }
                OpCode::StoreGlobal0 => {
                    let value = self.peek(0)?;
                    self.globals[21] = value;
                }
                OpCode::StoreGlobal1 => {
                    let value = self.peek(0)?;
                    self.globals[22] = value;
                }
                OpCode::StoreGlobal2 => {
                    let value = self.peek(0)?;
                    self.globals[23] = value;
                }
                OpCode::Call => {
                    let argc = chunk.read_byte(self.ip) as usize;
                    self.ip += 1;
                    let callee = self.peek(argc)?;
                    if callee.is_ptr() {
                        let obj = unsafe { &*callee.as_ptr() };
                        if let super::HeapData::String(name) = &obj.data {
                            let result = self.call_builtin(name, argc)?;
                            for _ in 0..=argc {
                                self.pop()?;
                            }
                            self.push(result)?;
                        } else if let super::HeapData::Function(func) = &obj.data {
                             if argc != func.arity as usize {
                                return Err(SpectreError::coded(ErrorCode::E012, "arity mismatch"));
                             }
                             let saved_ip = self.ip;
                             let saved_base = self.frame_base;
                             let base = self.stack.len() - argc;
                             self.ip = 0;
                             self.frame_base = base;
                             let result = self.execute_function_body(&func.chunk)?;
                             self.ip = saved_ip;
                             self.frame_base = saved_base;
                             for _ in 0..=argc { self.pop()?; }
                             self.push(result)?;
                        } else {
                            return Err(SpectreError::coded(ErrorCode::E011, "not callable in fn"));
                        }
                    } else {
                        return Err(SpectreError::coded(ErrorCode::E011, "not callable in fn"));
                    }
                }
                OpCode::Jump => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    self.ip += offset;
                }
                OpCode::JumpIfFalse => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    if !self.peek(0)?.is_truthy() {
                        self.ip += offset;
                    }
                }
                OpCode::Loop => {
                    let offset = chunk.read_u16(self.ip) as usize;
                    self.ip += 2;
                    self.ip -= offset;
                }
                OpCode::CheckIterLimit => {
                }
                _ => {
                    return Err(SpectreError::coded(ErrorCode::E004, format!("unsupported opcode in function: {:?}", op)));
                }
            }
        }
        Ok(NanBoxed::nil())
    }
    #[inline(always)]
    fn push(&mut self, value: NanBoxed) -> SpectreResult<()> {
        if self.stack.len() >= STACK_SIZE {
            return Err(SpectreError::coded(ErrorCode::E050, "stack"));
        }
        self.stack.push(value);
        Ok(())
    }
    #[inline(always)]
    fn pop(&mut self) -> SpectreResult<NanBoxed> {
        self.stack.pop().ok_or_else(|| SpectreError::coded(ErrorCode::E013, "empty stack"))
    }
    #[inline(always)]
    fn peek(&self, distance: usize) -> SpectreResult<NanBoxed> {
        if self.stack.len() <= distance {
            return Err(SpectreError::coded(ErrorCode::E013, "stack underflow"));
        }
        Ok(self.stack[self.stack.len() - 1 - distance])
    }
    fn value_to_nanbox(&self, value: &crate::interp::Value) -> NanBoxed {
        use crate::interp::Value;
        match value {
            Value::Number(n) => NanBoxed::number(*n),
            Value::Integer(n) => NanBoxed::integer(*n),
            Value::Float(f) => NanBoxed::number(*f),
            Value::Bool(b) => NanBoxed::boolean(*b),
            Value::Nil => NanBoxed::nil(),
            Value::String(s) => {
                let ptr = HeapObject::new_string(s);
                NanBoxed::ptr(ptr)
            }
            _ => NanBoxed::nil(), 
        }
    }
    fn values_equal(&self, a: NanBoxed, b: NanBoxed) -> bool {
        if a.bits() == b.bits() {
            return true;
        }
        if let (Some(na), Some(nb)) = (a.as_numeric(), b.as_numeric()) {
            return (na - nb).abs() < f64::EPSILON;
        }
        if a.is_ptr() && b.is_ptr() {
            debug_assert!(!a.as_ptr().is_null() && !b.as_ptr().is_null());
            let obj_a = unsafe { &*a.as_ptr() };
            let obj_b = unsafe { &*b.as_ptr() };
            if let (super::HeapData::String(sa), super::HeapData::String(sb)) = (&obj_a.data, &obj_b.data) {
                return sa == sb;
            }
        }
        false
    }
    fn call_builtin(&self, name: &str, argc: usize) -> SpectreResult<NanBoxed> {
        let mut args = Vec::with_capacity(argc);
        for i in 0..argc {
            args.push(self.peek(argc - 1 - i)?);
        }
        match name {
            "log" => {
                let output: Vec<_> = args.iter().map(|a| format!("{}", a)).collect();
                println!("{}", output.join(" "));
                Ok(NanBoxed::nil())
            }
            "typeof" => {
                if args.is_empty() {
                    return Err(SpectreError::coded(ErrorCode::E012, "typeof"));
                }
                let type_name = if args[0].is_nil() {
                    "nil"
                } else if args[0].is_bool() {
                    "bool"
                } else if args[0].is_number() {
                    "nb"
                } else if args[0].is_integer() {
                    "int"
                } else if args[0].is_ptr() {
                    let obj = unsafe { &*args[0].as_ptr() };
                    match &obj.data {
                        super::HeapData::String(_) => "wrd",
                        super::HeapData::List(_) => "lst",
                        super::HeapData::Map(_) => "map",
                        super::HeapData::Function(_) => "fn",
                    }
                } else {
                    "unknown"
                };
                let ptr = HeapObject::new_string(type_name);
                Ok(NanBoxed::ptr(ptr))
            }
            "sqrt" => {
                if args.is_empty() {
                    return Err(SpectreError::coded(ErrorCode::E012, "sqrt"));
                }
                let n = args[0].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "sqrt"))?;
                Ok(NanBoxed::number(n.sqrt()))
            }
            "abs" => {
                if args.is_empty() {
                    return Err(SpectreError::coded(ErrorCode::E012, "abs"));
                }
                if args[0].is_integer() {
                    Ok(NanBoxed::integer(args[0].as_integer().abs()))
                } else if args[0].is_number() {
                    Ok(NanBoxed::number(args[0].as_number().abs()))
                } else {
                    Err(SpectreError::coded(ErrorCode::E031, "abs"))
                }
            }
            "len" => {
                if args.is_empty() {
                    return Err(SpectreError::coded(ErrorCode::E012, "len"));
                }
                if args[0].is_ptr() {
                    let obj = unsafe { &*args[0].as_ptr() };
                    let len = match &obj.data {
                        super::HeapData::String(s) => s.len(),
                        super::HeapData::List(l) => l.len(),
                        super::HeapData::Map(m) => m.len(),
                        super::HeapData::Function(_) => 0,
                    };
                    Ok(NanBoxed::integer(len as i64))
                } else {
                    Err(SpectreError::coded(ErrorCode::E031, "len"))
                }
            }
            "floor" => {
                if args.is_empty() { return Err(SpectreError::coded(ErrorCode::E012, "floor")); }
                let n = args[0].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "floor"))?;
                Ok(NanBoxed::number(n.floor()))
            }
            "ceil" => {
                if args.is_empty() { return Err(SpectreError::coded(ErrorCode::E012, "ceil")); }
                let n = args[0].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "ceil"))?;
                Ok(NanBoxed::number(n.ceil()))
            }
            "round" => {
                if args.is_empty() { return Err(SpectreError::coded(ErrorCode::E012, "round")); }
                let n = args[0].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "round"))?;
                Ok(NanBoxed::number(n.round()))
            }
            "pow" => {
                if args.len() < 2 { return Err(SpectreError::coded(ErrorCode::E012, "pow")); }
                let base = args[0].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "pow"))?;
                let exp = args[1].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "pow"))?;
                Ok(NanBoxed::number(base.powf(exp)))
            }
            "sin" => {
                if args.is_empty() { return Err(SpectreError::coded(ErrorCode::E012, "sin")); }
                let n = args[0].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "sin"))?;
                Ok(NanBoxed::number(n.sin()))
            }
            "cos" => {
                if args.is_empty() { return Err(SpectreError::coded(ErrorCode::E012, "cos")); }
                let n = args[0].as_numeric().ok_or_else(|| SpectreError::coded(ErrorCode::E031, "cos"))?;
                Ok(NanBoxed::number(n.cos()))
            }
            _ => Err(SpectreError::coded(ErrorCode::E010, name)),
        }
    }
}
impl Default for VMNanBox {
    fn default() -> Self {
        Self::new()
    }
}
