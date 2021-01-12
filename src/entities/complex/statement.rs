use crate::entities::{complex::expression::Expression,
					  simple::ident::Ident};

use crate::parser::*;
use crate::{entities::{simple::literal::Literal}, Result, ExecuteExt};


#[derive(Debug, Clone)]
pub enum Statement {
    IfThen(IfThenStatement),
    IfElseThen(IfElseThenStatement),
    DoLoop(DoLoopStatement),
}

impl Parse for Statement {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
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
			},
			_ => unreachable!()
		}
	}
}

impl ExecuteExt for Statement {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		match self {
			Self::IfThen(stmt) => {
				stmt.execute(interpreter)?;
			},
			Self::IfElseThen(stmt) => {
				stmt.execute(interpreter)?;
			},
			Self::DoLoop(stmt) => {
				stmt.execute(interpreter)?;
			}
		}
		Ok(())
	}
}


#[derive(Debug, Clone)]
pub struct IfThenStatement {
	true_expr: Expression,
}

impl Parse for IfThenStatement {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut inner = pair.into_inner();
		Self {
			true_expr: Expression::parse(inner.nth(0).unwrap()),
		}
	}
}

impl ExecuteExt for IfThenStatement {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
			if crate::ForthInterpreter::bool(interpreter.get_last_literal()?) {
				self.true_expr.execute(interpreter)?;
			}
			Ok(())
	}
}


#[derive(Debug, Clone)]
pub struct IfElseThenStatement {
	true_expr: Expression,
	false_expr: Expression,
}

impl Parse for IfElseThenStatement {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut inner = pair.into_inner();
		Self {
			true_expr: Expression::parse(inner.nth(0).unwrap()),
			false_expr: Expression::parse(inner.nth(1).unwrap()),
		}
	}
}

impl ExecuteExt for IfElseThenStatement {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
			if crate::ForthInterpreter::bool(interpreter.get_last_literal()?) {
				self.true_expr.execute(interpreter)?;
			} else {
				self.false_expr.execute(interpreter)?;
			}
			Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct DoLoopStatement {
	counter: Ident,
	expr: Expression,
}

impl Parse for DoLoopStatement {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let mut inner = pair.into_inner();
		Self {
			counter: Ident::parse(inner.nth(0).unwrap()),
			expr: Expression::parse(inner.nth(1).unwrap()),
		}
	}
}

impl ExecuteExt for DoLoopStatement {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
		let (start, stop) = interpreter.get_binary_operands().unwrap();
		if let Literal::Integer(start) = start {
			if let Literal::Integer(stop) = stop {
				for i in start..stop {
					self.expr.execute(interpreter)?;
					interpreter.variables.insert(
						self.counter.to_string(),
						Some(Literal::Integer(i))
					);
				}
			}
		}
		Ok(())
	}
}
