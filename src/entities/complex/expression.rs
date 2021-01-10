use crate::entities::simple::{literal::Literal, ident::Ident};
use crate::parser::Parser;

pub enum ExpressionElement {
    Literal(Literal),
    Ident(Ident),
}

impl Parser for ExpressionElement {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		match pair.as_rule() {
			Rule::literal => {
				ExpressionElement::Literal(Literal::parse(pair))
			},
			Rule::ident => {
				ExpressionElement::Ident(Ident::parse(pair))
			},
		}
	}
}

pub struct Expression {
    elements: Vec<ExpressionElement>,
}

impl Parser for Expression {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let elements: Vec<ExpressionElement> = vec![];
		for inner_pair in pair.into_inner() {
			match inner_pair.as_rule() {
				Rule::literal => {
					elements.push(ExpressionElement::Literal(Literal::parse(inner_pair)));
				},
				Rule::ident => {
					elements.push(ExpressionElement::Ident(Ident::parse(inner_pair)));
				}
			}
		}
		Self {
			elements
		}
	}
}