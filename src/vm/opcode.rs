//! Bytecode instructions for the SpecterScript VM
//!
//! Stack-based instruction set. Each opcode operates on an implicit stack.
//! v1.0: Specialized opcodes for maximum performance.

/// Bytecode instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // === Stack Manipulation ===
    /// Push constant from pool: PUSH_CONST idx
    PushConst = 0,
    /// Push nil
    PushNil = 1,
    /// Push yes (true)
    PushTrue = 2,
    /// Push no (false)
    PushFalse = 3,
    /// Discard top of stack
    Pop = 4,
    /// Duplicate top of stack
    Dup = 5,

    // === Variables ===
    /// Load local variable: LOAD_LOCAL slot
    LoadLocal = 10,
    /// Store to local: STORE_LOCAL slot
    StoreLocal = 11,
    /// Load captured variable: LOAD_UPVALUE idx
    LoadUpvalue = 12,
    /// Store to captured: STORE_UPVALUE idx
    StoreUpvalue = 13,
    /// Load global by name index
    LoadGlobal = 14,
    /// Store global by name index
    StoreGlobal = 15,
    /// Define global (first assignment)
    DefineGlobal = 16,
    
    // === Specialized Local Access (no operand, faster) ===
    /// Load local slot 0 (no operand)
    LoadLocal0 = 17,
    /// Load local slot 1 (no operand)
    LoadLocal1 = 18,
    /// Load local slot 2 (no operand)
    LoadLocal2 = 19,

    // === Arithmetic (pop 2, push 1) ===
    Add = 20,
    Sub = 21,
    Mul = 22,
    Div = 23,
    Mod = 24,
    Pow = 25,
    /// Unary negate
    Neg = 26,
    /// Store to local slot 0 (no operand)
    StoreLocal0 = 27,
    /// Store to local slot 1 (no operand)
    StoreLocal1 = 28,
    /// Store to local slot 2 (no operand)
    StoreLocal2 = 29,

    // === Comparison (pop 2, push bool) ===
    Eq = 30,
    Ne = 31,
    Lt = 32,
    Gt = 33,
    Le = 34,
    Ge = 35,

    // === Logic ===
    /// Unary not
    Not = 40,
    /// Logical and (short-circuit)
    And = 41,
    /// Logical or (short-circuit)
    Or = 42,

    // === Control Flow ===
    /// Unconditional jump: JUMP offset
    Jump = 50,
    /// Pop and jump if false: JUMP_IF_FALSE offset
    JumpIfFalse = 51,
    /// Pop and jump if true: JUMP_IF_TRUE offset
    JumpIfTrue = 52,
    /// Jump backward (for loops): LOOP offset
    Loop = 53,

    // === Functions ===
    /// Call function: CALL argc
    Call = 60,
    /// Return from function
    Return = 61,
    /// Create closure: CLOSURE func_idx
    Closure = 62,

    // === Collections ===
    /// Create list: LIST n (pop n items)
    List = 70,
    /// Create map: MAP n (pop 2n items)
    Map = 71,
    /// Index access: pop index, pop collection, push item
    Index = 72,
    /// Index store: pop value, pop index, pop collection
    StoreIndex = 73,
    /// Get length of collection
    Len = 74,

    // === Iteration ===
    /// Initialize iterator from collection
    IterInit = 80,
    /// Get next value or jump if done
    IterNext = 81,

    // === Safety Checks ===
    /// Check iteration budget
    CheckIterLimit = 90,
    /// Check recursion depth
    CheckRecursion = 91,

    // === Error ===
    /// Throw error: THROW code
    Throw = 100,
    
    // === Specialized Integer Arithmetic (v1.0) ===
    /// Integer add (assumes both operands are integers)
    AddInt = 110,
    /// Integer subtract
    SubInt = 111,
    /// Integer multiply
    MulInt = 112,
    /// Increment local in-place: INC_LOCAL slot
    IncLocal = 113,
    /// Decrement local in-place: DEC_LOCAL slot
    DecLocal = 114,
    /// Add constant 1 to TOS
    Inc = 115,
    /// Subtract constant 1 from TOS
    Dec = 116,
}

impl OpCode {
    /// Returns the number of bytes this opcode's operand takes
    pub fn operand_size(self) -> usize {
        match self {
            // No operand
            OpCode::PushNil
            | OpCode::PushTrue
            | OpCode::PushFalse
            | OpCode::Pop
            | OpCode::Dup
            | OpCode::Add
            | OpCode::Sub
            | OpCode::Mul
            | OpCode::Div
            | OpCode::Mod
            | OpCode::Pow
            | OpCode::Neg
            | OpCode::Eq
            | OpCode::Ne
            | OpCode::Lt
            | OpCode::Gt
            | OpCode::Le
            | OpCode::Ge
            | OpCode::Not
            | OpCode::Return
            | OpCode::Index
            | OpCode::StoreIndex
            | OpCode::Len
            | OpCode::IterInit
            | OpCode::CheckIterLimit
            | OpCode::CheckRecursion
            // Specialized no-operand opcodes (v1.0)
            | OpCode::LoadLocal0
            | OpCode::LoadLocal1
            | OpCode::LoadLocal2
            | OpCode::StoreLocal0
            | OpCode::StoreLocal1
            | OpCode::StoreLocal2
            | OpCode::AddInt
            | OpCode::SubInt
            | OpCode::MulInt
            | OpCode::Inc
            | OpCode::Dec => 0,

            // 1-byte operand (index or count)
            OpCode::PushConst
            | OpCode::LoadLocal
            | OpCode::StoreLocal
            | OpCode::LoadUpvalue
            | OpCode::StoreUpvalue
            | OpCode::LoadGlobal
            | OpCode::StoreGlobal
            | OpCode::DefineGlobal
            | OpCode::Call
            | OpCode::Closure
            | OpCode::List
            | OpCode::Map
            | OpCode::IterNext
            | OpCode::Throw
            // Specialized 1-byte operand opcodes (v1.0)
            | OpCode::IncLocal
            | OpCode::DecLocal => 1,

            // 2-byte operand (jump offset)
            OpCode::Jump
            | OpCode::JumpIfFalse
            | OpCode::JumpIfTrue
            | OpCode::Loop
            | OpCode::And
            | OpCode::Or => 2,
        }
    }

    /// Convert from byte
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(OpCode::PushConst),
            1 => Some(OpCode::PushNil),
            2 => Some(OpCode::PushTrue),
            3 => Some(OpCode::PushFalse),
            4 => Some(OpCode::Pop),
            5 => Some(OpCode::Dup),
            10 => Some(OpCode::LoadLocal),
            11 => Some(OpCode::StoreLocal),
            12 => Some(OpCode::LoadUpvalue),
            13 => Some(OpCode::StoreUpvalue),
            14 => Some(OpCode::LoadGlobal),
            15 => Some(OpCode::StoreGlobal),
            16 => Some(OpCode::DefineGlobal),
            20 => Some(OpCode::Add),
            21 => Some(OpCode::Sub),
            22 => Some(OpCode::Mul),
            23 => Some(OpCode::Div),
            24 => Some(OpCode::Mod),
            25 => Some(OpCode::Pow),
            26 => Some(OpCode::Neg),
            30 => Some(OpCode::Eq),
            31 => Some(OpCode::Ne),
            32 => Some(OpCode::Lt),
            33 => Some(OpCode::Gt),
            34 => Some(OpCode::Le),
            35 => Some(OpCode::Ge),
            40 => Some(OpCode::Not),
            41 => Some(OpCode::And),
            42 => Some(OpCode::Or),
            50 => Some(OpCode::Jump),
            51 => Some(OpCode::JumpIfFalse),
            52 => Some(OpCode::JumpIfTrue),
            53 => Some(OpCode::Loop),
            60 => Some(OpCode::Call),
            61 => Some(OpCode::Return),
            62 => Some(OpCode::Closure),
            70 => Some(OpCode::List),
            71 => Some(OpCode::Map),
            72 => Some(OpCode::Index),
            73 => Some(OpCode::StoreIndex),
            74 => Some(OpCode::Len),
            80 => Some(OpCode::IterInit),
            81 => Some(OpCode::IterNext),
            90 => Some(OpCode::CheckIterLimit),
            91 => Some(OpCode::CheckRecursion),
            100 => Some(OpCode::Throw),
            // Specialized opcodes (v1.0)
            17 => Some(OpCode::LoadLocal0),
            18 => Some(OpCode::LoadLocal1),
            19 => Some(OpCode::LoadLocal2),
            27 => Some(OpCode::StoreLocal0),
            28 => Some(OpCode::StoreLocal1),
            29 => Some(OpCode::StoreLocal2),
            110 => Some(OpCode::AddInt),
            111 => Some(OpCode::SubInt),
            112 => Some(OpCode::MulInt),
            113 => Some(OpCode::IncLocal),
            114 => Some(OpCode::DecLocal),
            115 => Some(OpCode::Inc),
            116 => Some(OpCode::Dec),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_roundtrip() {
        for i in 0..=255u8 {
            if let Some(op) = OpCode::from_byte(i) {
                assert_eq!(op as u8, i);
            }
        }
    }
}
