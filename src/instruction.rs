use crate::global::Integer;

#[allow(non_snake_case, non_upper_case_globals)]
pub mod InstructionAsByte {
    pub const Push: u8 = 0;
    pub const Pop: u8 = 1;
    pub const Duplicate: u8 = 2;
    pub const Jump: u8 = 3;
    pub const JumpIf: u8 = 4;
    pub const Equal: u8 = 5;
    pub const Add: u8 = 6;
    pub const Subtract: u8 = 7;
    pub const Multipy: u8 = 8;
    pub const Divide: u8 = 9;
    pub const Dump: u8 = 10;
    pub const Halt: u8 = 11;
}

pub enum InstructionType {
    Push,
    Pop,
    Duplicate,
    Jump,
    JumpIf,

    Equal,
    Add,
    Subtract,
    Multipy,
    Divide,
    Dump,
    Halt,
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
