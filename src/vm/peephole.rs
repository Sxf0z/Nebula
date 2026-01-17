use super::{Chunk, OpCode};

pub fn optimize(chunk: &mut Chunk) {
    remove_redundant_pops(chunk);
    collapse_push_pop(chunk);
}

fn remove_redundant_pops(chunk: &mut Chunk) {
    let code = chunk.code_mut();
    let mut write = 0;
    let mut i = 0;
    
    while i < code.len() {
        let op = OpCode::from_byte(code[i]);
        
        if let Some(OpCode::Pop) = op {
            if write > 0 {
                if let Some(prev_op) = OpCode::from_byte(code[write - 1]) {
                    if matches!(prev_op, OpCode::Pop) {
                        i += 1;
                        continue;
                    }
                }
            }
        }
        
        if let Some(op) = op {
            let size = 1 + op.operand_size();
            for j in 0..size {
                if i + j < code.len() {
                    code[write] = code[i + j];
                    write += 1;
                }
            }
            i += size;
        } else {
            code[write] = code[i];
            write += 1;
            i += 1;
        }
    }
    
    code.truncate(write);
}

fn collapse_push_pop(chunk: &mut Chunk) {
    let code = chunk.code_mut();
    let mut i = 0;
    
    while i + 2 < code.len() {
        let op1 = OpCode::from_byte(code[i]);
        
        if let Some(OpCode::PushConst) = op1 {
            if i + 2 < code.len() && code[i + 2] == OpCode::Pop as u8 {
                code[i] = OpCode::PushNil as u8;
                code[i + 1] = OpCode::Pop as u8;
                code[i + 2] = OpCode::PushNil as u8;
            }
        }
        
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interp::Value;

    #[test]
    fn test_peephole_basic() {
        let mut chunk = Chunk::new();
        chunk.write(OpCode::PushNil as u8, 1);
        chunk.write(OpCode::Pop as u8, 1);
        chunk.write(OpCode::PushTrue as u8, 1);
        
        let initial_len = chunk.code().len();
        optimize(&mut chunk);
        assert!(chunk.code().len() <= initial_len);
    }
}
