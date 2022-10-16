#[derive(Debug)]
pub enum Trap {
    StackUnderflow,
    DivisionByZero,
    InvalidInstructionPointer,
}
