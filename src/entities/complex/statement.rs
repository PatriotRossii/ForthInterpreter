use crate::entities::{complex::expression::Expression, simple::ident::Ident};

use crate::parser::{Parse, Rule};
use crate::{entities::simple::literal::Literal, ExecuteExt, Result};

#[derive(Debug, Clone)]
pub enum Statement {
    IfThen(IfThenStatement),
    IfElseThen(IfElseThenStatement),
    DoLoop(DoLoopStatement),
}

impl Parse for Statement {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::if_then_statement => Self::IfThen(IfThenStatement::parse(inner)),
            Rule::if_else_then_statement => Self::IfElseThen(IfElseThenStatement::parse(inner)),
            Rule::do_loop => Self::DoLoop(DoLoopStatement::parse(inner)),
            _ => unreachable!(),
        }
    }
}

impl ExecuteExt for Statement {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        match self {
            Self::IfThen(stmt) => {
                stmt.execute(interpreter)?;
            }
            Self::IfElseThen(stmt) => {
                stmt.execute(interpreter)?;
            }
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
            true_expr: Expression::parse(inner.next().unwrap()),
        }
    }
}

impl ExecuteExt for IfThenStatement {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        if interpreter.get_last_literal()?.into() {
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
            true_expr: Expression::parse(inner.next().unwrap()),
            false_expr: Expression::parse(inner.next().unwrap()),
        }
    }
}

impl ExecuteExt for IfElseThenStatement {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        if interpreter.get_last_literal()?.into() {
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
            counter: Ident::parse(inner.next().unwrap()),
            expr: Expression::parse(inner.next().unwrap()),
        }
    }
}

impl ExecuteExt for DoLoopStatement {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        let (start, stop) = interpreter.get_binary_operands().unwrap();
        if let Literal::Integer(start) = start {
            if let Literal::Integer(stop) = stop {
                for i in start..stop {
                    self.expr.execute(interpreter)?;
                    interpreter.set_variable(self.counter.name(), Literal::Integer(i));
                }
            }
        }
        Ok(())
    }
}
