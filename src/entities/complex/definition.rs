use crate::entities::simple::literal::Literal;
use crate::stack::Stack;
use crate::Result;


pub enum Definition {
    Variable(VariableDefinition),
    Constant(ConstantDefinition),
    Word(WordDefinition),
}

pub struct VariableDefinition {
    name: String,
    value: Literal,
}

pub struct ConstantDefinition {
    name: String,
    value: Literal,
}

pub struct WordDefinition {
    name: String,
    value: fn(&mut Stack<Literal>) -> Result<()>,
}