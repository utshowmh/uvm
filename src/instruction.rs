use crate::global::Word;

#[derive(Debug)]
pub enum InstructionType {
    Push,
    Jump,
    Hult,

    Plus,
    Minus,
    Mult,
    Division,
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operand: Option<Word>,
}

impl Instruction {
    pub fn new(instruction_type: InstructionType, operand: Option<Word>) -> Self {
        Self {
            instruction_type,
            operand,
        }
    }
}
