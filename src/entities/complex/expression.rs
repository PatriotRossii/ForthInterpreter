use crate::entities::simple::{literal::Literal, ident::Ident};

pub enum ExpressionElement {
    Literal(Literal),
    Ident(Ident),
}

pub struct Expression {
    elements: Vec<ExpressionElement>,
}