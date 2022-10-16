use crate::{
    global::Word,
    instruction::{Instruction, InstructionType},
    trap::Trap,
};

pub struct UVM {
    stack_capacity: usize,
    stack: Vec<Word>,
}

impl UVM {
    pub fn new(stack_capacity: usize) -> Self {
        Self {
            stack_capacity,
            stack: Vec::new(),
        }
    }

    pub fn dump(&self) {
        println!("stack: {:#?}", self.stack);
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) -> Option<Trap> {
        match instruction.instruction_type {
            InstructionType::Push => {
                if self.stack.len() >= self.stack_capacity {
                    return Some(Trap::StackOverflow);
                }
                self.stack.push(instruction.operand.unwrap());
            }
            InstructionType::Plus => {
                if self.stack.len() < 2 {
                    return Some(Trap::StackUndererflow);
                }
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a + b);
            }
        }
        None
    }
}
