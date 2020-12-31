#![feature(assoc_char_funcs)]

mod literal;
mod stack;
mod errors;

use stack::Stack;
use literal::Literal;
use errors::ForthError::{self, StackUnderflow, InvalidOperands};

type Result<T> = std::result::Result<T, ForthError>;

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

pub struct ForthInterpreter {
	stack: Stack<Literal>,
}

impl ForthInterpreter {
	pub fn new() -> Self {
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
		Ok(())
	}

	fn drop(&mut self) -> Result<()> {
		self.stack.pop().ok_or(StackUnderflow)?;
		Ok(())
	}

	fn swap(&mut self) -> Result<()> {
		let length = self.stack.length();
		if length >= 2 {
			self.stack.swap(length - 2, length - 1);
			Ok(())
		} else {
			Err(StackUnderflow)
		}
	}

	fn over(&mut self) -> Result<()> {
		let length = self.stack.length();
		if length >= 2 {
			self.push(self.stack.get(length - 2));
			return Ok(())
		}
		Err(StackUnderflow)
	}

	fn rot(&mut self) -> Result<()> {
		let length = self.stack.length();
		if length >= 3 {
			let element = self.stack.remove(length - 3);
			self.stack.push(element);
			return Ok(())
		}
		Err(StackUnderflow)
	}

	fn print_top(&mut self) -> Result<()> {
		print!("{}", *self.stack.last().ok_or(StackUnderflow)?);
		Ok(())
	}

	fn emit(&mut self) -> Result<()> {
		let last = *self.stack.last().ok_or(StackUnderflow)?;
		if let Literal::Integer(i) = last {
			print!("{}", char::from_u32(i as u32).ok_or(InvalidOperands)?);
		}
		Ok(())
	}

	fn cr(&mut self) -> Result<()> {
		print!("\n");
		Ok(())
	}

	fn equal(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		self.stack.push(Literal::Integer(ternary!(a == b, -1, 0)));
		Ok(())
	}

	fn less_than(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		self.stack.push(Literal::Integer(ternary!(a < b, -1, 0)));
		Ok(())
	}

	fn greater_than(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		self.stack.push(Literal::Integer(ternary!(a > b, -1, 0)));
		Ok(())
	}

	fn and(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		self.stack.push(Literal::Integer(ternary!(a != 0.into() && b != 0.into(), -1, 0)));
		Ok(())
	}

	fn or(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		self.stack.push(Literal::Integer(ternary!(a != 0.into() || b != 0.into(), -1, 0)));
		Ok(())
	}
	
	fn invert(&mut self) -> Result<()> {
		let a = self.stack.pop().ok_or(StackUnderflow)?;
		self.stack.push(Literal::Integer(ternary!(a == 0.into(), -1, 0)));
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
