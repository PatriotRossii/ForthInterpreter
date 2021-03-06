use crate::entities::complex::{expression::Expression, statement::Statement};
use crate::entities::simple::{ident::Ident, literal::Literal};

use crate::parser::{Parse, Rule};
use crate::{ExecuteExt, Result};

#[derive(Debug, Clone)]
pub enum Definition {
    Variable(Variable),
    Constant(Constant),
    Word(Word),
}

impl Parse for Definition {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::variable_definition => Self::Variable(Variable::parse(inner)),
            Rule::constant_definition => Self::Constant(Constant::parse(inner)),
            Rule::word_definition => Self::Word(Word::parse(inner)),
            _ => unreachable!(),
        }
    }
}

impl ExecuteExt for Definition {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        match self {
            Self::Variable(variable) => {
                variable.execute(interpreter)?;
            }
            Self::Constant(constant) => {
                constant.execute(interpreter)?;
            }
            Self::Word(word) => {
                word.execute(interpreter)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: Ident,
    value: Option<Literal>,
}

impl Parse for Variable {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let name = pair.into_inner().next().unwrap();
        Self {
            name: Ident::parse(name),
            value: None,
        }
    }
}

impl ExecuteExt for Variable {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        interpreter.variables.push(crate::Variable {
            name: self.name.to_string(),
            value: None,
        });
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Constant {
    name: Ident,
    value: Literal,
}

impl Parse for Constant {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut value: Option<Literal> = None;
        let mut var_name: Option<Ident> = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ident => {
                    var_name = Some(Ident::parse(inner_pair));
                }
                Rule::literal => {
                    value = Some(inner_pair.as_str().parse::<i64>().unwrap().into());
                }
                _ => unreachable!(),
            }
        }

        Self {
            name: var_name.unwrap(),
            value: value.unwrap(),
        }
    }
}

impl ExecuteExt for Constant {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        interpreter
            .constants
            .insert(self.name.to_string(), self.value.clone());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum WordElement {
    Statement(Statement),
    Expression(Expression),
}

impl ExecuteExt for WordElement {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        match self {
            Self::Statement(stmt) => {
                stmt.execute(interpreter)?;
            }
            Self::Expression(expr) => {
                expr.execute(interpreter)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    name: Ident,
    value: Vec<WordElement>,
}

impl Parse for Word {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut inner_pair = pair.into_inner();
        let name = Ident::parse(inner_pair.next().unwrap());
        let value = {
            let mut word_elements = vec![];
            let pair = inner_pair.next().unwrap();
            match pair.as_rule() {
                Rule::expression => {
                    word_elements.push(WordElement::Expression(Expression::parse(pair)))
                }
                Rule::statement => {
                    word_elements.push(WordElement::Statement(Statement::parse(pair)))
                }
                _ => unreachable!(),
            }
            word_elements
        };

        Self { name, value }
    }
}

impl ExecuteExt for Word {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        interpreter
            .user_words
            .insert(self.name.to_string(), self.value.clone());
        Ok(())
    }
}
