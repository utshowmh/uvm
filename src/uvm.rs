use crate::{
    global::Word,
    instruction::{Instruction, InstructionType},
    trap::Trap,
};

pub struct UVM {
    stack: Vec<Word>,
    program: Vec<Instruction>,
    pub ip: usize,
    pub halt: bool,
}

impl UVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            program: Vec::new(),
            ip: 0,
            halt: false,
        }
    }

    pub fn dump_stack(&self) {
        println!("stack: {:?}", self.stack);
    }

    pub fn load_program_from_memory(&mut self, program: &mut Vec<Instruction>) {
        self.program.append(program);
    }

    pub fn execute_instruction(&mut self) -> Option<Trap> {
        if self.ip >= self.program.len() {
            return Some(Trap::InvalidInstructionPointer);
        }

        let instruction = &self.program[self.ip];

        match instruction.instruction_type {
            InstructionType::Push => {
                self.ip += 1;

                self.stack.push(instruction.operand.unwrap());
            }
            InstructionType::Plus => {
                self.ip += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a + b);
            }
            InstructionType::Minus => {
                self.ip += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a - b);
            }
            InstructionType::Mult => {
                self.ip += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a * b);
            }
            InstructionType::Division => {
                self.ip += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();

                if b == 0 {
                    return Some(Trap::DivisionByZero);
                }

                self.stack.push(a / b);
            }
            InstructionType::Jump => {
                if let Some(jump_to) = instruction.operand {
                    self.ip = jump_to as usize;
                }
            }
            InstructionType::Hult => {
                self.halt = true;
            }
        }
        None
    }
}
