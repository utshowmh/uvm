#[derive(Debug)]
pub enum UVMError {
    StackUnderflow,
    DivisionByZero,
    InvalidInstructionPointer,
    IllegalOperation,
    IllegalOperand,
}
