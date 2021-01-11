use crate::entities::simple::{literal::Literal, ident::Ident};
use crate::entities::complex::{statement::Statement,
							   expression::Expression};


use crate::parser::*;
use crate::{ExecuteExt, Result};

#[derive(Debug, Clone)]
pub enum Definition {
    Variable(Variable),
    Constant(Constant),
    Word(Word),
}

impl Parse for Definition {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let inner = pair.into_inner().nth(0).unwrap();
		match inner.as_rule() {
			Rule::variable_definition => {
				Definition::Variable(Variable::parse(inner))
			},
			Rule::constant_definition => {
				Definition::Constant(Constant::parse(inner))
			},
			Rule::word_definition => {
				Definition::Word(Word::parse(inner))
			},
			_ => unreachable!()
		}
	}
}

impl ExecuteExt for Definition {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		match self {
			Self::Variable(variable) => {
				variable.execute(interpreter);
			},
			Self::Constant(constant) => {
				constant.execute(interpreter);
			},
			Self::Word(word) => {
				word.execute(interpreter);
			}
		}
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: Ident,
    value: Option<Literal>,
}

impl Parse for Variable {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
    	Self {
    		name: Ident::parse(pair),
    		value: None,
    	}
    }
}

impl ExecuteExt for Variable {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		interpreter.variables.insert(
			self.name.to_string(),
			None,
		);
		Ok(())
	}
}
#[derive(Debug, Clone)]
pub struct Constant {
    name: Ident,
    value: Literal,
}

impl Parse for Constant {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut value: Option<Literal> = None;
		let mut var_name: Option<Ident> = None;

		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::ident => {
					var_name = Some(Ident::parse(inner_pair));
				}
				Rule::literal => {
					value = Some(inner_pair.as_str().into());
				}
				_ => unreachable!()
			}
		}

		Self {
			name: var_name.unwrap(),
			value: value.unwrap(),
		}
	}
}

impl ExecuteExt for Constant {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		interpreter.constants.insert(
			self.name.to_string(),
			self.value.clone(),
		);
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub enum WordElement {
	Statement(Statement),
	Expression(Expression),
}

impl ExecuteExt for WordElement {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		match self {
			Self::Statement(stmt) => {
				stmt.execute(interpreter);
			},
			Self::Expression(expr) => {
				expr.execute(interpreter);
			},
		}
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct Word {
    name: Ident,
    value: WordElement,
}

impl Parse for Word {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut name: Option<Ident> = None;
		let mut value: Option<WordElement> = None;

		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::ident => {
					name = Some(Ident::parse(inner_pair));
				}
				Rule::expression => {
					value = Some(WordElement::Expression(Expression::parse(inner_pair)));
				},
				Rule::statement => {
					value = Some(WordElement::Statement(Statement::parse(inner_pair)));
				}
				_ => unreachable!()	
			}
		}
		Self {
			name: name.unwrap(),
			value: value.unwrap(),
		}
	}
}

impl ExecuteExt for Word {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		interpreter.user_words.insert(
			self.name.to_string(),
			self.value.clone()
		);
		Ok(())
	}
}
