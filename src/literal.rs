use std::{cmp::Ordering, fmt::{self, Display}};

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
			&Literal::String(s) => {
				write!(f, "{}", s)
			}
			&Literal::Unknown => {
				write!(f, "")
			},
		}
    }
}

impl PartialEq for Literal {
	fn eq(&self, other: &Self) -> bool {
		match self {
			&Literal::Integer(i) => {
				if let &Literal::Integer(j) = other {
					i == j
				} else {
					false
				}
			},
			&Literal::String(s) => {
				if let &Literal::String(os) = other {
					s == os
				} else {
					false
				}
			}
			&Literal::Unknown => {
				if let &Literal::Unknown = other {
					true
				} else {
					false
				}
			}
		}
	}
}

impl PartialOrd for Literal {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self {
			&Literal::Integer(i) => {
				if let &Literal::Integer(j) = other {
					i.partial_cmp(&j)
				} else {
					None
				}
			},
			&Literal::String(s) => {
				if let &Literal::String(os) = other {
					s.partial_cmp(os)
				} else {
					None
				}
			}
			&Literal::Unknown => {
				if let &Literal::Unknown = other {
					None
				} else {
					None
				}
			}
		}
	}
}