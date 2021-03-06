use cpython::{PyString, Python, ToPyObject};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use crate::entities::complex::array::Array;
use crate::parser::{Parse, Rule};
use crate::{ExecuteExt, Result};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub struct Pointer {
    pub address: usize,
    pub offset: usize,
}

impl Pointer {
    #[inline]
    pub fn new(address: usize, offset: usize) -> Self {
        Self { address, offset }
    }
}

type PointerType = Pointer;
type IntegerType = i64;
type StringType = std::string::String;

type ArrayType = Array;

#[derive(Debug, Clone, Eq, Hash)]
pub enum Literal {
    Pointer(PointerType),
    Integer(IntegerType),
    String(StringType),

    Array(ArrayType),

    Unknown,
}

impl ExecuteExt for Literal {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        interpreter.stack.push(self.clone());
        Ok(())
    }
}

impl Parse for Literal {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::integer => Self::Integer(inner.as_str().parse::<i64>().unwrap()),
            Rule::string => Self::String(inner.as_str().to_string()),
            _ => unreachable!(),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => {
                write!(f, "{}", i)
            }
            Self::String(s) => {
                write!(f, "{}", s)
            }
            Self::Pointer(i) => {
                write!(f, "{:?}", i)
            }
            Self::Array(vec) => {
                write!(f, "{:?}", vec)
            }
            Self::Unknown => {
                write!(f, "")
            }
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Literal::Integer(i) => {
                if let Literal::Integer(j) = *other {
                    *i == j
                } else {
                    false
                }
            }
            Literal::Pointer(i) => {
                if let Literal::Pointer(j) = other {
                    i == j
                } else {
                    false
                }
            }
            Literal::String(s) => {
                if let Literal::String(os) = other {
                    s == os
                } else {
                    false
                }
            }
            Literal::Array(arr) => {
                if let Literal::Array(oa) = other {
                    arr == oa
                } else {
                    false
                }
            }
            Literal::Unknown => {
                matches!(other, &Literal::Unknown)
            }
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Integer(i) => {
                if let Literal::Integer(j) = *other {
                    i.partial_cmp(&j)
                } else {
                    None
                }
            }
            Self::Pointer(i) => {
                if let Literal::Pointer(j) = other {
                    i.partial_cmp(j)
                } else {
                    None
                }
            }
            Self::String(s) => {
                if let Literal::String(os) = other {
                    s.partial_cmp(os)
                } else {
                    None
                }
            }
            Self::Array(arr) => {
                if let Literal::Array(oarr) = other {
                    arr.partial_cmp(oarr)
                } else {
                    None
                }
            }
            Self::Unknown => {
                if let Literal::Unknown = other {
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
        Self::Integer(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl ToPyObject for Literal {
    type ObjectType = PyString;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        match self {
            Literal::Integer(i) => PyString::new(py, &i.to_string()),
            Literal::String(i) => PyString::new(py, i.as_str()),
            _ => unreachable!(),
        }
    }
}

impl From<&Literal> for bool {
    fn from(value: &Literal) -> Self {
        match value {
            &Literal::Integer(e) => e == -1_i64,
            Literal::String(_) => true,
            _ => unreachable!(),
        }
    }
}
