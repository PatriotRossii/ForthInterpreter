#![feature(assoc_char_funcs)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod entities;

mod stack;
mod errors;
pub mod parser;

use std::{cell::RefCell, collections::{HashMap, hash_map::{Entry}}, hash::Hash};

use stack::Stack;
use entities::{simple::literal::Literal, complex::definition::WordElement};
use errors::ForthError::{self, StackUnderflow, InvalidOperands, VariableNotExist};

use pest::Parser;
use parser::*;

type Result<T> = std::result::Result<T, ForthError>;

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

type WordFn = fn(&mut ForthInterpreter) -> Result<()>;

trait ExecuteExt {
	fn execute(&mut self, interpreter: &mut ForthInterpreter) -> Result<()>;
}

pub struct ForthInterpreter {
	stack: Stack<Literal>,
	variables: HashMap<String, Option<Literal>>,
	constants: HashMap<String, Literal>, // No need in Option cause constant is initialized always

	native_words: HashMap<String, WordFn>,
	user_words: HashMap<String, WordElement>,
}

impl ForthInterpreter {
	pub fn new() -> Self {
		let words: HashMap<i32, i32> = [(1, 2), (1, 2), (2, 3)].iter().cloned().collect();
		Self {
			stack: Stack::new(),
			variables: HashMap::new(),
			constants: HashMap::new(),

			native_words: [
				("+".into(), ForthInterpreter::add as WordFn), ("-".into(), ForthInterpreter::sub),
				("*".into(), ForthInterpreter::mul), ("/".into(), ForthInterpreter::div),
				("dup".into(), ForthInterpreter::dup), ("drop".into(), ForthInterpreter::drop),
				("swap".into(), ForthInterpreter::swap), ("over".into(), ForthInterpreter::over),
				("rot".into(), ForthInterpreter::rot), (".".into(), ForthInterpreter::print_top),
				("emit".into(), ForthInterpreter::emit), ("cr".into(), ForthInterpreter::cr),
				("=".into(), ForthInterpreter::equal), ("<".into(), ForthInterpreter::less_than),
				(">".into(), ForthInterpreter::greater_than), ("invert".into(), ForthInterpreter::invert),
				("and".into(), ForthInterpreter::and), ("or".into(), ForthInterpreter::or)
			].iter().cloned().collect(),
			user_words: HashMap::<String, WordElement>::new(),
		}
	}
	
	fn get_binary_operands(&mut self) -> Result<(Literal, Literal)> {
		let b: Literal = self.stack.pop().ok_or(StackUnderflow)?;
		let a: Literal = self.stack.pop().ok_or(StackUnderflow)?;
		Ok((a, b))
	}

	pub fn get_last_literal(&self) -> Result<&Literal> {
		Ok(self.stack.last().ok_or(StackUnderflow)?)
	}

	pub fn get_stack_dump(&self) -> &Stack<Literal> {
		&self.stack
	}

	pub fn bool(literal: &Literal) -> bool {
		match &literal {
			&Literal::Integer(i) => {
				!(*i != -1i64)
			},
			Literal::String(_) => {
				true
			},
			_ => unreachable!()
		}
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
		self.push(self.get_last_literal()?.clone());
		Ok(())
	}

	fn drop(&mut self) -> Result<()> {
		self.stack.pop().ok_or(StackUnderflow)?;
		Ok(())
	}

	fn swap(&mut self) -> Result<()> {
		let (a, b) = self.get_binary_operands()?;
		self.push(b);
		self.push(a);
		Ok(())
	}

	fn over(&mut self) -> Result<()> {
		let length = self.stack.length();
		if length >= 2 {
			self.push((*self.stack.get(length - 2)).clone());
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
		let last = self.stack.last().ok_or(StackUnderflow)?;
		if let &Literal::Integer(i) = last {
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

	fn variable(&mut self) -> Result<()> {
		let var_name = self.stack.pop().ok_or(StackUnderflow)?;
		self.variables.insert(var_name.to_string(), None);
		Ok(())
	}

	fn store_variable(&mut self) -> Result<()> {
		let (var_value, var_name) = self.get_binary_operands()?;
		let var_name = var_name.to_string();

		if  !self.variables.contains_key(&var_name) {
			return Err(VariableNotExist);
		}
		self.variables.insert(var_name, Some(var_value));
		Ok(())
	}

	fn get_variable(&mut self) -> Result<()> {
		let var_name = self.stack.pop().ok_or(VariableNotExist)?.to_string();
		if !self.variables.contains_key(&var_name) {
			return Err(VariableNotExist);
		}
		self.stack.push(self.variables[&var_name].clone().unwrap());
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

	#[test]
	fn test_parsing() {
		use crate::ForthInterpreter;
		
		let mut forth: ForthInterpreter = ForthInterpreter::new();
		//forth.execute_line("a b +")
	}
}
