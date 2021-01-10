use crate::entities::{complex::expression::Expression,
					  simple::ident::Ident};
use crate::parser::Parser;

pub enum Statement {
    IfThen(IfThenStatement),
    IfElseThen(IfElseThenStatement),
    DoLoop(DoLoopStatement),
}

impl Parser for Statement {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let inner = pair.into_inner().nth(0).unwrap();
		match inner.as_rule() {
			Rule::if_then_statement => {
				Statement::IfThen(IfThenStatement::parse(inner))
			},
			Rule::if_else_then_statement => {
				Statement::IfElseThen(IfElseThenStatement::parse(inner))
			},
			Rule::do_loop => {
				Statement::DoLoop(DoLoopStatement::parse(inner))
			}
		}
	}
}

pub struct IfThenStatement {
	true_expr: Expression,
}

impl Parser for IfThenStatement {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let inner = pair.into_inner();
		Self {
			true_expr: Expression::parse(inner.nth(0).unwrap()),
		}
	}
}

pub struct IfElseThenStatement {
	true_expr: Expression,
	false_expr: Expression,
}

impl Parser for IfElseThenStatement {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let inner = pair.into_inner();
		Self {
			true_expr: Expression::parse(inner.nth(0).unwrap()),
			false_expr: Expression::parse(inner.nth(1).unwrap()),
		}
	}
}

pub struct DoLoopStatement {
	counter: Ident,
	expr: Expression,
}

impl Parser for DoLoopStatement {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		let ident: Option<Ident> = None;
		let expr: Option<Expression> = None;

		let inner = pair.into_inner();
		Self {
			counter: Ident::parse(inner.nth(0).unwrap()),
			expr: Expression::parse(inner.nth(1).unwrap()),
		}
	}
}
