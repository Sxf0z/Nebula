use super::OpCode;
use crate::interp::Value;
#[derive(Debug, Clone)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}
impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::with_capacity(256),
            constants: Vec::with_capacity(16),
            lines: Vec::with_capacity(256),
        }
    }
    pub fn write_op(&mut self, op: OpCode, line: usize) {
        self.code.push(op as u8);
        self.lines.push(line);
    }
    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }
    pub fn write_u16(&mut self, value: u16, line: usize) {
        self.code.push((value >> 8) as u8);
        self.lines.push(line);
        self.code.push((value & 0xff) as u8);
        self.lines.push(line);
    }
    pub fn add_constant(&mut self, value: Value) -> u8 {
        for (i, c) in self.constants.iter().enumerate() {
            if values_equal(c, &value) {
                return i as u8;
            }
        }
        let idx = self.constants.len();
        if idx > 255 {
            return 255;
        }
        self.constants.push(value);
        idx as u8
    }
    pub fn len(&self) -> usize {
        self.code.len()
    }
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.code.len().saturating_sub(offset).saturating_sub(2);
        let jump = jump.min(u16::MAX as usize);
        self.code[offset] = (jump >> 8) as u8;
        self.code[offset + 1] = (jump & 0xff) as u8;
    }
    pub fn read_byte(&self, offset: usize) -> u8 {
        self.code[offset]
    }
    pub fn read_u16(&self, offset: usize) -> u16 {
        ((self.code[offset] as u16) << 8) | (self.code[offset + 1] as u16)
    }
    pub fn get_constant(&self, idx: u8) -> &Value {
        &self.constants[idx as usize]
    }
    pub fn get_line(&self, offset: usize) -> usize {
        self.lines.get(offset).copied().unwrap_or(0)
    }
    pub fn code(&self) -> &[u8] {
        &self.code
    }
}
impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => (x - y).abs() < f64::EPSILON,
        (Value::Integer(x), Value::Integer(y)) => x == y,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Nil, Value::Nil) => true,
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chunk_write() {
        let mut chunk = Chunk::new();
        chunk.write_op(OpCode::PushConst, 1);
        chunk.write_byte(0, 1);
        chunk.write_op(OpCode::Return, 1);
        assert_eq!(chunk.len(), 3);
        assert_eq!(chunk.read_byte(0), OpCode::PushConst as u8);
    }
    #[test]
    fn test_constant_dedup() {
        let mut chunk = Chunk::new();
        let idx1 = chunk.add_constant(Value::Number(42.0));
        let idx2 = chunk.add_constant(Value::Number(42.0));
        assert_eq!(idx1, idx2);
    }
}
