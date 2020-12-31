mod literal;
mod stack;
mod errors;

use stack::Stack;
use literal::Literal;
use errors::ForthError::{self, StackUnderflow, InvalidOperands};

type Result<T> = std::result::Result<T, ForthError>;

struct ForthInterpreter {
	stack: Stack<Literal>,
}

impl ForthInterpreter {
	fn new() -> Self {
		Self {
			stack: Stack::new()
		}
	}
	
	fn get_binary_operands(&mut self) -> Result<(Literal, Literal)> {
		let b: Literal = self.stack.pop().ok_or(StackUnderflow)?;
		let a: Literal = self.stack.pop().ok_or(StackUnderflow)?;
		Ok((a, b))
	}

	fn get_last_literal(&self) -> Result<&Literal> {
		Ok(self.stack.last().ok_or(StackUnderflow)?)
	}		

	fn add(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		if let Literal::Integer(a) = a {
			if let Literal::Integer(b) = b {
				self.push(Literal::Integer(a + b));
				return Ok(())
			}
		}
		Err(InvalidOperands)
	}

	fn sub(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		if let Literal::Integer(a) = a {
			if let Literal::Integer(b) = b {
				self.push(Literal::Integer(a - b));
				return Ok(())
			}
		}
		Err(InvalidOperands)
	}

        fn mul(&mut self) -> Result<()> {
                let (a, b) = self.get_binary_operands()?;
                if let Literal::Integer(a) = a {
                        if let Literal::Integer(b) = b {
                                self.push(Literal::Integer(a * b));
                                return Ok(()) 
                        }
                }
                Err(InvalidOperands)
        }

        fn div(&mut self) -> Result<()> {
                let (a, b) = self.get_binary_operands()?;
                if let Literal::Integer(a) = a {
                        if let Literal::Integer(b) = b {
                                self.push(Literal::Integer(a / b));
                                return Ok(()) 
                        }
                }
                Err(InvalidOperands)
        }

	fn dup(&mut self) -> Result<()> {
		self.push(*self.get_last_literal()?);
		println!("{:?}", self.stack);
		Ok(())
	}

	fn push(&mut self, value: Literal) {
		self.stack.push(value);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_interpreter() {
		use crate::ForthInterpreter;
		use crate::Literal;

		let mut forth: ForthInterpreter = ForthInterpreter::new();
		forth.push(Literal::Integer(5));
		forth.push(Literal::Integer(5));
	}
}
