use crate::entities::simple::{literal::Literal, ident::Ident};
use crate::parser::*;

use crate::{ExecuteExt, Result};

pub enum ExpressionElement {
    Literal(Literal),
    Ident(Ident),
}

impl Parse for ExpressionElement {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		match pair.as_rule() {
			Rule::literal => {
				ExpressionElement::Literal(Literal::parse(pair))
			},
			Rule::ident => {
				ExpressionElement::Ident(Ident::parse(pair))
			},
			_ => unreachable!()
		}
	}
}

impl ExecuteExt for ExpressionElement {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		match &mut self {
			Self::Literal(literal) => {
				literal.execute(interpreter);
			},
			Self::Ident(ident) => {
				ident.execute(interpreter);
			}
		}
		Ok(())
	}
}


pub struct Expression {
    elements: Vec<ExpressionElement>,
}

impl Parse for Expression {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut elements: Vec<ExpressionElement> = vec![];
		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::literal => {
					elements.push(ExpressionElement::Literal(Literal::parse(inner_pair)));
				},
				Rule::ident => {
					elements.push(ExpressionElement::Ident(Ident::parse(inner_pair)));
				},
				_ => unreachable!()
			}
		}
		Self {
			elements
		}
	}
}

impl ExecuteExt for Expression {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		for element in &self.elements {
			element.execute(interpreter);
		}
		Ok(())
	}
}
