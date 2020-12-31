use thiserror::Error;

#[derive(Error, Debug)]
pub enum ForthError {
	#[error("stack underflow")]
	StackUnderflow,
}
