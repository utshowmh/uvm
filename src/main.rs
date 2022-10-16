mod global;
mod instruction;
mod trap;
mod uvm;

use instruction::{Instruction, InstructionType};
use uvm::UVM;

fn main() {
    let mut program = vec![
        Instruction::new(InstructionType::Push, Some(20)),
        Instruction::new(InstructionType::Push, Some(20)),
        Instruction::new(InstructionType::Plus, None),
        Instruction::new(InstructionType::Push, Some(10)),
        Instruction::new(InstructionType::Minus, None),
        Instruction::new(InstructionType::Push, Some(2)),
        Instruction::new(InstructionType::Mult, None),
        Instruction::new(InstructionType::Jump, Some(10)),
        Instruction::new(InstructionType::Push, Some(0)),
        Instruction::new(InstructionType::Division, None),
        Instruction::new(InstructionType::Hult, None),
    ];

    let mut vm = UVM::new();
    vm.load_program_from_memory(&mut program);
    while !vm.halt {
        if let Some(trap) = vm.execute_instruction() {
            println!("TRAP: {:#?}", trap);
            std::process::exit(1);
        }
        vm.dump_stack();
    }
}
