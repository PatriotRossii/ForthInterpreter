use thiserror::Error;

#[derive(Error, Debug)]
pub enum ForthError {
    #[error("stack underflow")]
    StackUnderflow,
    #[error("invalid operands")]
    InvalidOperands,
    #[error("There are no such variable")]
    VariableNotExist,
    #[error("Index out of bound")]
    IndexOutOfBound,
    #[error("Other error")]
    OtherError,
}
