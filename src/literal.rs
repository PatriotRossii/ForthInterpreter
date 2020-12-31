use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum Literal {
	Integer(i64),
	String(&'static str),
	Unknown,
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			&Literal::Integer(i) => {
				write!(f, "{}", i)
			},
			&Literal::Unknown => {
				write!(f, "")
			},
			&Literal::String(s) => {
				write!(f, "{}", s)
			}
		}
    }
}