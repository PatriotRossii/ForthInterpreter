pub mod simple;
pub mod complex;

use simple::literal::Literal;
use complex::{definition::Definition, expression::Expression};

enum Line {
    Definition(Definition),
    Expression(Expression),
}