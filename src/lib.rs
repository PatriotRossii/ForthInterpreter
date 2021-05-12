#![warn(clippy::pedantic)]
#![feature(hash_raw_entry)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod entities;

mod errors;
pub mod parser;
mod stack;
mod tests;
pub mod words;

use std::{collections::HashMap, convert::TryInto};

use entities::{
    complex::{array::Array, definition::WordElement, variable::Variable},
    simple::literal::{Literal, Pointer},
};
use errors::ForthError::{self, InvalidOperands, StackUnderflow};
use stack::Stack;

use pest::Parser;

use parser::{ForthParser, Parse, Rule};
use words::{IOWords, LogicWords, MathWords, OtherWords, StackWords, StandardWords};

use console::Term;

type Result<T> = std::result::Result<T, ForthError>;

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

type WordFn = fn(&mut ForthInterpreter) -> Result<()>;

trait ExecuteExt {
    fn execute(&self, interpreter: &mut ForthInterpreter) -> Result<()>;
}

const CELL_SIZE: i64 = 1;

pub struct ForthInterpreter {
    stack: Stack<Literal>,

    terminal: console::Term,

    variables: Vec<Variable>,
    constants: HashMap<String, Literal>, // No need in Option cause constant is initialized always

    native_words: HashMap<String, WordFn>,
    user_words: HashMap<String, Vec<WordElement>>,
}

impl MathWords for crate::ForthInterpreter {
    fn add_ptr_offset(pointer: Pointer, offset: i64) -> Pointer {
        Pointer {
            offset: pointer.offset + offset as usize,
            ..pointer
        }
    }

    fn add(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        match a {
            Literal::Integer(a) => {
                if let Literal::Integer(b) = b {
                    self.push((a + b).into());
                    return Ok(());
                }
                if let Literal::Pointer(b) = b {
                    self.push(Literal::Pointer(Self::add_ptr_offset(b, a)));
                    return Ok(());
                }
            }
            Literal::Pointer(a) => {
                if let Literal::Integer(offset) = b {
                    self.push(Literal::Pointer(Self::add_ptr_offset(a, offset)));
                    return Ok(());
                }
            }
            _ => {}
        }
        Err(InvalidOperands)
    }

    fn sub(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        if let Literal::Integer(a) = a {
            if let Literal::Integer(b) = b {
                self.push(Literal::Integer(a - b));
                return Ok(());
            }
        }
        Err(InvalidOperands)
    }

    fn mul(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        if let Literal::Integer(a) = a {
            if let Literal::Integer(b) = b {
                self.push(Literal::Integer(a * b));
                return Ok(());
            }
        }
        Err(InvalidOperands)
    }

    fn div(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        if let Literal::Integer(a) = a {
            if let Literal::Integer(b) = b {
                self.push(Literal::Integer(a / b));
                return Ok(());
            }
        }
        Err(InvalidOperands)
    }

    fn r#mod(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        if let Literal::Integer(a) = a {
            if let Literal::Integer(b) = b {
                self.push(Literal::Integer(a % b));
                return Ok(());
            }
        }
        Err(InvalidOperands)
    }

    fn negate(&mut self) -> Result<()> {
        let a = self.get_unary_operand()?;
        if let Literal::Integer(a) = a {
            self.push(Literal::Integer(-a));
            return Ok(());
        }
        Err(InvalidOperands)
    }

    fn abs(&mut self) -> Result<()> {
        let a = self.get_unary_operand()?;
        if let Literal::Integer(a) = a {
            self.push(Literal::Integer(a.abs()));
            return Ok(());
        }
        Err(InvalidOperands)
    }

    fn max(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        if let Literal::Integer(a) = a {
            if let Literal::Integer(b) = b {
                self.push(Literal::Integer(a.max(b)));
                return Ok(());
            }
        }
        Err(InvalidOperands)
    }

    fn min(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        if let Literal::Integer(a) = a {
            if let Literal::Integer(b) = b {
                self.push(Literal::Integer(a.min(b)));
                return Ok(());
            }
        }
        Err(InvalidOperands)
    }

    fn add_to(&mut self) -> Result<()> {
        let (value, pointer) = self.get_binary_operands()?;

        self.push(pointer.clone());
        self.fetch_variable()?;
        self.push(value);

        self.add()?;
        self.push(pointer);
        self.store_variable()?;

        Ok(())
    }

    fn sub_from(&mut self) -> Result<()> {
        let (value, pointer) = self.get_binary_operands()?;

        self.push(pointer.clone());
        self.fetch_variable()?;
        self.push(value);

        self.sub()?;
        self.push(pointer);
        self.store_variable()?;

        Ok(())
    }

    fn mul_by(&mut self) -> Result<()> {
        let (value, pointer) = self.get_binary_operands()?;

        self.push(pointer.clone());
        self.fetch_variable()?;
        self.push(value);

        self.mul()?;
        self.push(pointer);
        self.store_variable()?;

        Ok(())
    }

    fn div_by(&mut self) -> Result<()> {
        let (value, pointer) = self.get_binary_operands()?;

        self.push(pointer.clone());
        self.fetch_variable()?;
        self.push(value);

        self.div()?;
        self.push(pointer);
        self.store_variable()?;

        Ok(())
    }
}

impl IOWords for crate::ForthInterpreter {
    fn print_top(&mut self) -> Result<()> {
        print!("{} ", *self.stack.last().ok_or(StackUnderflow)?);
        Ok(())
    }

    fn emit(&mut self) -> Result<()> {
        let last = self.stack.last().ok_or(StackUnderflow)?;
        if let Literal::Integer(i) = *last {
            print!("{}", char::from_u32(i as u32).ok_or(InvalidOperands)?);
        }
        Ok(())
    }

    fn cr(&mut self) -> Result<()> {
        println!();
        Ok(())
    }

    fn key(&mut self) -> Result<()> {
        if let Ok(ch) = self.terminal.read_char() {
            self.push(Literal::Integer(ch as i64));
        }
        Ok(())
    }

    fn word(&mut self) -> Result<()> {
        let (storage, del_code) = self.get_binary_operands()?;
        if let Literal::Integer(code) = del_code {
            let delimiter = char::from_u32(code as u32).unwrap();
            if let Literal::Pointer(ptr) = storage {
                let pointer_storage = self.variables[ptr.address].get_mut().unwrap();
                if let Literal::Array(arr) = pointer_storage {
                    while let Ok(ch) = self.terminal.read_char() {
                        if ch != delimiter {
                            arr.push(Literal::Integer(ch as i64))?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl LogicWords for crate::ForthInterpreter {
    fn equal(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        self.push(Literal::Integer(ternary!(a == b, -1, 0)));
        Ok(())
    }

    fn greater(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        self.push(Literal::Integer(ternary!(a > b, -1, 0)));
        Ok(())
    }

    fn less(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        self.push(Literal::Integer(ternary!(a < b, -1, 0)));
        Ok(())
    }

    fn not(&mut self) -> Result<()> {
        let a = self.stack.pop().ok_or(StackUnderflow)?;
        self.push(Literal::Integer(ternary!(a == 0.into(), -1, 0)));
        Ok(())
    }

    fn and(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        self.push(Literal::Integer(ternary!(
            a != 0.into() && b != 0.into(),
            -1,
            0
        )));
        Ok(())
    }

    fn or(&mut self) -> Result<()> {
        let (a, b) = self.get_binary_operands()?;
        self.push(Literal::Integer(ternary!(
            a != 0.into() || b != 0.into(),
            -1,
            0
        )));
        Ok(())
    }
}

impl StackWords for crate::ForthInterpreter {
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
            return Ok(());
        }
        Err(StackUnderflow)
    }

    fn rot(&mut self) -> Result<()> {
        let length = self.stack.length();
        if length >= 3 {
            let element = self.stack.remove(length - 3);
            self.stack.push(element);
            return Ok(());
        }
        Err(StackUnderflow)
    }

    fn fetch_variable(&mut self) -> Result<()> {
        let var_index = self.get_unary_operand()?;
        
        if let Literal::Pointer(idx) = var_index {
            if idx.offset == 0 {
                self.push(
                    self.variables[idx.address]
                        .value
                        .as_ref()
                        .unwrap_or(&0_i64.into())
                        .clone(),
                );
            } else {
                if let Some(Literal::Array(arr)) = &self.variables[idx.address].value {
                    let variable = arr
                        .get(idx.offset)
                        .ok_or(ForthError::IndexOutOfBound)?
                        .clone();

                    self.push(variable);
                }
            }
        }
        Ok(())
    }
}

impl OtherWords for crate::ForthInterpreter {
    fn store_variable(&mut self) -> Result<()> {
        let (var_value, var_index) = self.get_binary_operands()?;
        if let Literal::Pointer(ptr) = var_index {
            let address = ptr.address;
            let offset = ptr.offset;

            let variable = self.variables.get_mut(address).unwrap();
            let var_storage = variable.get_mut();

            match var_storage {
                Some(Literal::Array(arr)) => {
                    arr.set(offset, var_value)?;
                }
                _ => {
                    variable.value = Some(var_value);
                }
            }
        }
        Ok(())
    }

    fn cells(&mut self) -> Result<()> {
        let count = self.get_unary_operand()?;
        if let Literal::Integer(count) = count {
            self.push(Literal::Integer(CELL_SIZE * count));
        }
        Ok(())
    }

    fn allot(&mut self) -> Result<()> {
        let count_of_elements = self.get_unary_operand()?;

        if let Literal::Integer(count_of_elements) = count_of_elements {
            let variable = self.variables.last_mut().unwrap();
            let array = Array::new((count_of_elements + 1).try_into().unwrap());
            variable.value = Some(Literal::Array(array));
        }
        Ok(())
    }
}

impl StandardWords for ForthInterpreter {}

impl Default for ForthInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl ForthInterpreter {
    #[inline]
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            variables: Vec::new(),
            constants: HashMap::new(),

            terminal: Term::stdout(),

            native_words: <Self as StandardWords>::get_words(),
            user_words: HashMap::<String, Vec<WordElement>>::new(),
        }
    }

    fn get_unary_operand(&mut self) -> Result<Literal> {
        self.stack.pop().ok_or(StackUnderflow)
    }

    fn get_binary_operands(&mut self) -> Result<(Literal, Literal)> {
        let b: Literal = self.stack.pop().ok_or(StackUnderflow)?;
        let a: Literal = self.stack.pop().ok_or(StackUnderflow)?;
        Ok((a, b))
    }

    #[inline]
    pub fn get_last_literal(&self) -> Result<&Literal> {
        self.stack.last().ok_or(StackUnderflow)
    }

    #[inline]
    pub fn get_stack_dump(&self) -> &Stack<Literal> {
        &self.stack
    }

    #[inline]
    pub fn get_vars_dump(&self) -> &Vec<Variable> {
        &self.variables
    }

    #[inline]
    pub fn get_consts_dump(&self) -> &HashMap<String, Literal> {
        &self.constants
    }

    #[inline]
    pub fn get_native_words_dump(&self) -> &HashMap<String, WordFn> {
        &self.native_words
    }

    #[inline]
    pub fn get_user_words_dump(&self) -> &HashMap<String, Vec<WordElement>> {
        &self.user_words
    }

    fn set_variable(&mut self, name: &str, value: Literal) {
        let variable = self.variables.iter_mut().find(|var| var.name == name);
        match variable {
            None => self.variables.push(Variable {
                name: name.to_string(),
                value: Some(value),
            }),
            Some(e) => {
                e.value = Some(value);
            }
        }
    }

    fn contains_variable(&self, name: &str) -> bool {
        !matches!(self.variables.iter().find(|var| var.name == name), None)
    }

    fn get_variable_id(&self, name: &str) -> Option<usize> {
        self.variables.iter().position(|var| var.name == name)
    }

    #[inline]
    pub fn execute_line(&mut self, line: &str) -> Result<()> {
        let line_pair = ForthParser::parse(Rule::line, line).unwrap();
        let line = entities::Line::parse(line_pair.peek().unwrap());

        line.execute(self)?;

        Ok(())
    }

    #[inline]
    pub fn execute(&mut self, text: &str) -> Result<()> {
        for line in text.lines() {
            self.execute_line(line)?;
        }
        Ok(())
    }

    #[inline]
    pub fn clear_state(&mut self) {
        *self = Self::new();
    }

    fn push(&mut self, value: Literal) {
        self.stack.push(value);
    }
}

#[cfg(test)]
mod interpreter_tests {
    use crate::ForthInterpreter;
    use crate::Literal;

    #[test]
    fn test_interpreter() {
        let mut forth: ForthInterpreter = ForthInterpreter::new();
        forth.push(Literal::Integer(5));
        forth.push(Literal::Integer(5));
    }

    #[test]
    fn test_parsing() {
        let mut forth: ForthInterpreter = ForthInterpreter::new();
        //forth.execute_line("a b +")
    }

    #[test]
    fn test_variable() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute_line("variable user_var").unwrap();
        interpreter.execute_line("123 user_var !").unwrap();
        interpreter.execute_line("user_var").unwrap();

        let last = interpreter.get_last_literal().unwrap();

        match last {
            Literal::Pointer(ptr) => {
                assert_eq!(ptr.address, 0);
                assert_eq!(ptr.offset, 0);
            }
            _ => assert!(false),
        }
    }
}
