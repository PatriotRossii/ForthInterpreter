pub mod simple;
pub mod complex;

use simple::literal::Literal;
use complex::{definition::Definition, expression::Expression};
use crate::parser::Parser;

enum Line {
    Definition(Definition),
    Expression(Expression),
}


impl Parser for Line {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let inner = pair.into_inner().nth(0).unwrap();
		match inner.as_rule() {
			Rule::definition => {
				Line::Definition(Definition::parse(inner))
			},
			Rule::expression => {
				Line::Expression(Expression::parse(inner))
			},
		}
	}
}