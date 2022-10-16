use crate::global::Word;

pub enum InstructionType {
    Push,
    Plus,
}

pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operand: Option<Word>,
}

impl Instruction {
    pub fn push(operand: Option<Word>) -> Self {
        Self {
            instruction_type: InstructionType::Push,
            operand,
        }
    }

    pub fn plus() -> Self {
        Self {
            instruction_type: InstructionType::Plus,
            operand: None,
        }
    }
}
