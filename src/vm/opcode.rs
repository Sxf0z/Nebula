#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    PushConst = 0,
    PushNil = 1,
    PushTrue = 2,
    PushFalse = 3,
    Pop = 4,
    Dup = 5,
    LoadLocal = 10,
    StoreLocal = 11,
    LoadUpvalue = 12,
    StoreUpvalue = 13,
    LoadGlobal = 14,
    StoreGlobal = 15,
    DefineGlobal = 16,
    LoadLocal0 = 17,
    LoadLocal1 = 18,
    LoadLocal2 = 19,
    Add = 20,
    Sub = 21,
    Mul = 22,
    Div = 23,
    Mod = 24,
    Pow = 25,
    Neg = 26,
    StoreLocal0 = 27,
    StoreLocal1 = 28,
    StoreLocal2 = 29,
    Eq = 30,
    Ne = 31,
    Lt = 32,
    Gt = 33,
    Le = 34,
    Ge = 35,
    Not = 40,
    And = 41,
    Or = 42,
    Jump = 50,
    JumpIfFalse = 51,
    JumpIfTrue = 52,
    Loop = 53,
    Call = 60,
    Return = 61,
    Closure = 62,
    List = 70,
    Map = 71,
    Index = 72,
    StoreIndex = 73,
    Len = 74,
    IterInit = 80,
    IterNext = 81,
    CheckIterLimit = 90,
    CheckRecursion = 91,
    Throw = 100,
    AddInt = 110,
    SubInt = 111,
    MulInt = 112,
    IncLocal = 113,
    DecLocal = 114,
    Inc = 115,
    Dec = 116,
    LoadGlobal0 = 120,
    LoadGlobal1 = 121,
    LoadGlobal2 = 122,
    StoreGlobal0 = 123,
    StoreGlobal1 = 124,
    StoreGlobal2 = 125,
    CallBuiltin = 130,
}
impl OpCode {
    pub fn operand_size(self) -> usize {
        match self {
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
            | OpCode::Dec
            | OpCode::LoadGlobal0
            | OpCode::LoadGlobal1
            | OpCode::LoadGlobal2
            | OpCode::StoreGlobal0
            | OpCode::StoreGlobal1
            | OpCode::StoreGlobal2 => 0,
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
            | OpCode::IncLocal
            | OpCode::DecLocal
            | OpCode::CallBuiltin => 2,
            OpCode::Jump
            | OpCode::JumpIfFalse
            | OpCode::JumpIfTrue
            | OpCode::Loop
            | OpCode::And
            | OpCode::Or => 2,
        }
    }
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
            120 => Some(OpCode::LoadGlobal0),
            121 => Some(OpCode::LoadGlobal1),
            122 => Some(OpCode::LoadGlobal2),
            123 => Some(OpCode::StoreGlobal0),
            124 => Some(OpCode::StoreGlobal1),
            125 => Some(OpCode::StoreGlobal2),
            130 => Some(OpCode::CallBuiltin),
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
