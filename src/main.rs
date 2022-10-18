mod global;
mod instruction;
mod trap;
mod uvm;

use std::env::args;

use uvm::UVM;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let mut vm = UVM::new();
        vm.run(&args[1]);
    } else {
        eprintln!(
            "
Program: UVM

Usage:
    <source_path>: executes the (given) file.
        "
        );
    }
}
