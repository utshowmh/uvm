use instruction::Instruction;
use uvm::UVM;

mod global;
mod instruction;
mod trap;
mod uvm;

fn main() {
    let mut vm = UVM::new(1024);
    let program = vec![
        Instruction::push(Some(1)),
        Instruction::push(Some(2)),
        Instruction::push(Some(3)),
        Instruction::plus(),
    ];
    for instr in program {
        if let Some(trap) = vm.execute_instruction(instr) {
            eprintln!("Got Traped: {:#?}", trap);
            std::process::exit(1);
        }
        vm.dump();
    }
}
