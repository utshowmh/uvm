use std::fs::read_to_string;

use crate::{
    global::Integer,
    instruction::{Instruction, InstructionAsByte, InstructionType},
    trap::Trap,
};

pub struct UVM {
    stack: Vec<Integer>,
    program: Vec<Instruction>,
    instruction_pointer: usize,
    halt: bool,
}

impl UVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            program: Vec::new(),
            instruction_pointer: 0,
            halt: false,
        }
    }

    pub fn run(&mut self, filepath: &str) {
        if let Some(trap) = self.load_program_from_file(filepath) {
            eprintln!("Trap: {:#?}", trap);
            std::process::exit(1);
        };
        while !self.halt {
            if let Some(trap) = self.execute_instruction() {
                eprintln!("Trap: {:#?}", trap);
                std::process::exit(1);
            };
        }
    }

    fn load_program_from_file(&mut self, filepath: &str) -> Option<Trap> {
        let instructions = match read_to_string(filepath) {
            Ok(instruction_bytes) => instruction_bytes,
            Err(err) => {
                eprintln!("Error: {:#?}", err);
                std::process::exit(1);
            }
        };
        let instructions = instructions.split("\n");

        for instruction in instructions {
            let instruction: Vec<&str> = instruction.split(" ").collect();
            match instruction.len() {
                1 => {
                    let instruction_type: u8 = instruction[0].trim().parse().unwrap();
                    match instruction_type {
                        InstructionAsByte::Add => self
                            .program
                            .push(Instruction::new(InstructionType::Add, None)),
                        InstructionAsByte::Subtract => self
                            .program
                            .push(Instruction::new(InstructionType::Subtract, None)),
                        InstructionAsByte::Multipy => self
                            .program
                            .push(Instruction::new(InstructionType::Multipy, None)),
                        InstructionAsByte::Divide => self
                            .program
                            .push(Instruction::new(InstructionType::Divide, None)),
                        InstructionAsByte::Equal => self
                            .program
                            .push(Instruction::new(InstructionType::Equal, None)),
                        InstructionAsByte::Dump => self
                            .program
                            .push(Instruction::new(InstructionType::Dump, None)),
                        InstructionAsByte::Hult => self
                            .program
                            .push(Instruction::new(InstructionType::Hult, None)),
                        _ => {
                            return Some(Trap::IllegalOperation);
                        }
                    }
                }
                2 => {
                    let instruction_type: u8 = instruction[0].trim().parse().unwrap();
                    let operand: i64 = match instruction[1].trim().parse() {
                        Ok(operand) => operand,
                        Err(_) => return Some(Trap::IllegalOperand),
                    };
                    match instruction_type {
                        InstructionAsByte::Push => self
                            .program
                            .push(Instruction::new(InstructionType::Push, Some(operand))),
                        InstructionAsByte::Duplicate => self
                            .program
                            .push(Instruction::new(InstructionType::Duplicate, Some(operand))),
                        InstructionAsByte::Jump => self
                            .program
                            .push(Instruction::new(InstructionType::Jump, Some(operand))),
                        InstructionAsByte::JumpIf => self
                            .program
                            .push(Instruction::new(InstructionType::JumpIf, Some(operand))),
                        _ => {
                            return Some(Trap::IllegalOperation);
                        }
                    }
                }
                _ => {
                    return Some(Trap::IllegalOperation);
                }
            }
        }
        None
    }

    fn execute_instruction(&mut self) -> Option<Trap> {
        if self.instruction_pointer >= self.program.len() {
            return Some(Trap::InvalidInstructionPointer);
        }

        let instruction = &self.program[self.instruction_pointer];

        match instruction.instruction_type {
            InstructionType::Push => {
                self.instruction_pointer += 1;

                if let Some(value) = instruction.operand {
                    self.stack.push(value);
                } else {
                    return Some(Trap::IllegalOperand);
                }
            }
            InstructionType::Duplicate => {
                self.instruction_pointer += 1;

                if let Some(instruction_pointer) = instruction.operand {
                    let stack_length = self.stack.len() as i64;
                    if stack_length - instruction_pointer < 1 {
                        return Some(Trap::StackUnderflow);
                    }
                    if instruction_pointer < 0 {
                        return Some(Trap::IllegalOperand);
                    } else {
                        self.stack
                            .push(self.stack[(stack_length - 1 - instruction_pointer) as usize]);
                    }
                }
            }
            InstructionType::Add => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a + b);
            }
            InstructionType::Subtract => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a - b);
            }
            InstructionType::Multipy => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a * b);
            }
            InstructionType::Divide => {
                self.instruction_pointer += 1;

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
            InstructionType::Equal => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(Trap::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push((a == b) as i64);
            }
            InstructionType::Jump => {
                if let Some(jump_to) = instruction.operand {
                    self.instruction_pointer = jump_to as usize;
                } else {
                    return Some(Trap::IllegalOperand);
                }
            }
            InstructionType::JumpIf => {
                self.instruction_pointer += 1;

                if self.stack.len() < 1 {
                    return Some(Trap::StackUnderflow);
                }

                let a = self.stack.pop().unwrap() != 0;
                if let Some(jump_to) = instruction.operand {
                    if a {
                        self.instruction_pointer = jump_to as usize;
                    }
                } else {
                    return Some(Trap::IllegalOperand);
                }
            }
            InstructionType::Dump => {
                self.instruction_pointer += 1;

                if self.stack.len() < 1 {
                    return Some(Trap::StackUnderflow);
                }

                let a = self.stack.pop().unwrap();
                println!("{}", a);
            }
            InstructionType::Hult => {
                self.halt = true;
            }
        }
        None
    }
}
