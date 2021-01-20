pub mod complex;
pub mod simple;

use complex::{definition::Definition, expression::Expression};

use crate::{parser::*, ExecuteExt, ForthInterpreter, Result};

#[derive(Debug, Clone)]
pub enum Line {
    Definition(Definition),
    Expression(Expression),
}

impl Parse for Line {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let inner = pair.into_inner().nth(0).unwrap();
        match inner.as_rule() {
            Rule::definition => Line::Definition(Definition::parse(inner)),
            Rule::expression => Line::Expression(Expression::parse(inner)),
            _ => unreachable!(),
        }
    }
}

impl ExecuteExt for Line {
    fn execute(&mut self, interpreter: &mut ForthInterpreter) -> Result<()> {
        match self {
            Self::Definition(e) => e.execute(interpreter)?,
            Self::Expression(e) => e.execute(interpreter)?,
        }
        Ok(())
    }
}
