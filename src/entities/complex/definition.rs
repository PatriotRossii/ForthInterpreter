use crate::entities::simple::literal::Literal;
use crate::entities::complex::{statement::Statement,
							   expression::Expression};

use crate::parser::*;

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

pub struct Variable {
    name: String,
    value: Option<Literal>,
}

impl Parse for Variable {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
    	Self {
    		name: pair.as_str().to_string(),
    		value: None,
    	}
    }
}

pub struct Constant {
    name: String,
    value: Literal,
}

impl Parse for Constant {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut value: Option<Literal> = None;
		let mut var_name: Option<String> = None;

		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::ident => {
					var_name = Some(inner_pair.as_str().to_string());
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

pub enum WordElement {
	Statement(Statement),
	Expression(Expression),
}

pub struct Word {
    name: String,
    value: WordElement,
}

impl Parse for Word {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut name: Option<String> = None;
		let mut value: Option<WordElement> = None;

		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::ident => {
					name = Some(inner_pair.as_str().to_string());
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