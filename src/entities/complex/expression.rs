use crate::entities::simple::{ident::Ident, literal::Literal};
use crate::parser::*;

use crate::{ExecuteExt, Result};

#[derive(Debug, Clone)]
pub enum ExpressionElement {
    Literal(Literal),
    Ident(Ident),
}

impl Parse for ExpressionElement {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::literal => ExpressionElement::Literal(Literal::parse(pair)),
            Rule::ident => ExpressionElement::Ident(Ident::parse(pair)),
            _ => unreachable!(),
        }
    }
}

impl ExecuteExt for ExpressionElement {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        match self {
            Self::Literal(literal) => {
                literal.execute(interpreter)?;
            }
            Self::Ident(ident) => {
                ident.execute(interpreter)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
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
                }
                Rule::ident => {
                    elements.push(ExpressionElement::Ident(Ident::parse(inner_pair)));
                }
                _ => unreachable!(),
            }
        }
        Self { elements }
    }
}

impl ExecuteExt for Expression {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        for element in &self.elements {
            element.execute(interpreter)?;
        }
        Ok(())
    }
}
