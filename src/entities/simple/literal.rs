use std::{cmp::Ordering, fmt::{self, Display}};
use cpython::{Python, ToPyObject, PyString};

use crate::parser::*;
use crate::{ExecuteExt, Result};

type PointerType = u64;
type IntegerType = i64;
type StringType = std::string::String;

#[derive(Debug, Clone, Eq, Hash)]
pub enum Literal {
	Pointer(PointerType),
	Integer(IntegerType),
	String(StringType),
	Unknown,
}

impl ExecuteExt for Literal {
	fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
			interpreter.stack.push(self.clone());
			Ok(())
	}
}

impl Parse for Literal {
	fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
		let inner = pair.into_inner().nth(0).unwrap();
		match inner.as_rule() {
			Rule::integer => {
				Literal::Integer(inner.as_str().parse::<i64>().unwrap())
			},
			Rule::string => {
				Literal::String(inner.as_str().to_string())
			},
			_ => unreachable!()
		}
	}
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			&Literal::Integer(i) => {
				write!(f, "{}", i)
			},
			&Literal::String(s) => {
				write!(f, "{}", s)
			}
			&Literal::Pointer(i) => {
				write!(f, "{}", i)
			}
			&Literal::Unknown => {
				write!(f, "")
			},
		}
    }
}

impl PartialEq for Literal {
	fn eq(&self, other: &Self) -> bool {
		match &self {
			&Literal::Integer(i) => {
				if let &Literal::Integer(j) = other {
					*i == j
				} else {
					false
				}
			},
			&Literal::Pointer(i) => {
				if let &Literal::Pointer(j) = other {
					*i == j
				} else {
					false
				}
			},
			&Literal::String(s) => {
				if let Literal::String(os) = other {
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
		match &self {
			&Literal::Integer(i) => {
				if let &Literal::Integer(j) = other {
					i.partial_cmp(&j)
				} else {
					None
				}
			},
			&Literal::Pointer(i) => {
				if let &Literal::Pointer(j) = other {
					i.partial_cmp(&j)
				} else {
					None
				}
			}
			&Literal::String(s) => {
				if let Literal::String(os) = other {
					s.partial_cmp(&os)
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

impl From<i64> for Literal {
	fn from(value: i64) -> Self {
		Literal::Integer(value)
	}
}

impl From<&str> for Literal {
	fn from(value: &str) -> Self {
		Literal::String(value.into())
	}
}

impl From<String> for Literal {
	fn from(value: String) -> Self {
		Literal::String(value)
	}
}

impl ToPyObject for Literal {
	type ObjectType = PyString;
	fn to_py_object(&self, py: Python) -> Self::ObjectType {
		match &self {
			&Literal::Integer(i) => {
				PyString::new(py, &i.to_string())
			},
			&Literal::String(i) => {
				PyString::new(py, i.as_str())
			},
			_ => unreachable!()
		}
	}
}