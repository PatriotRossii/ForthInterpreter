use crate::entities::simple::literal::Literal;
use crate::stack::Stack;
use crate::Result;

use crate::parser::Parse;

pub enum Definition {
    Variable(Variable),
    Constant(Constant),
    Word(Word),
}

impl Parser for Definition {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
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
		}
	}
}

pub struct Variable {
    name: String,
    value: Literal,
}

impl Parser for Variable {
    fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
    	Self {
    		name: pair.as_str().into_string(),
    		value: None,
    	}
    }
}

pub struct Constant {
    name: String,
    value: Literal,
}

impl Parser for Constant {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let mut value: Option<Literal> = None;
		let mut var_name: Option<String> = None;

		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::ident => {
					var_name = Some(inner_pair.as_str().into_string());
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
	Statment(Statement),
	Expression(Expression),
}

pub struct Word {
    name: String,
    value: WordElement,
}

impl Parser for Word {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let mut name: Option<String> = None;
		let mut value: Option<WordElement> = None;

		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::ident => {
					name = Some(inner_pair.as_str().into_string());
				}
				Rule::expression => {
					value = Some(WordElement::Expression::parse(inner_pair));
				},
				Rule::statement => {
					value = Some(WordElement::Statement::parse(inner_pair));
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