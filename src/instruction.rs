use crate::global::Integer;

#[allow(non_snake_case, non_upper_case_globals)]
pub mod InstructionAsByte {
    pub const Push: u8 = 0;
    pub const Duplicate: u8 = 1;
    pub const Jump: u8 = 2;
    pub const JumpIf: u8 = 3;

    pub const Equal: u8 = 4;

    pub const Add: u8 = 5;
    pub const Subtract: u8 = 6;
    pub const Multipy: u8 = 7;
    pub const Divide: u8 = 8;

    pub const Dump: u8 = 9;
    pub const Hult: u8 = 10;
}

pub enum InstructionType {
    Push,
    Duplicate,
    Jump,
    JumpIf,

    Equal,
    Add,
    Subtract,
    Multipy,
    Divide,
    Dump,
    Hult,
}

pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operand: Option<Integer>,
}

impl Instruction {
    pub fn new(instruction_type: InstructionType, operand: Option<Integer>) -> Self {
        Self {
            instruction_type,
            operand,
        }
    }
}
