use crate::entities::{complex::expression::Expression,
					  simple::ident::Ident};

pub enum Statement {
    IfThen(IfThenStatement),
    IfElseThen(IfElseThenStatement),
    DoLoop(DoLoopStatement),
}

pub struct IfThenStatement {
	true_expr: Expression,
}

pub struct IfElseThenStatement {
	true_expr: Expression,
	false_expr: Expression,
}

pub struct DoLoopStatement {
	counter: Ident,
	expr: Expression,
}