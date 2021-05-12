extern crate nom;
use nom::{AsChar, IResult, branch::alt, bytes::complete::{take_while}, combinator::map_res};

use crate::InterpreterError;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
}

fn from_decimal(input: &str) -> Result<i64, std::num::ParseIntError> {
    i64::from_str_radix(input, 10)
}

fn is_decimal_digit(c: char) -> bool {
    c.is_dec_digit()
}

fn decimal_number(input: &str) -> IResult<&str, i64> {
    map_res(
        take_while(is_decimal_digit),
        from_decimal
    )(input)
}

fn integer(input: &str) -> IResult<&str, Literal> {
    let (input, number) = decimal_number(input)?;
    Ok((input, Literal::Integer(number)))
}

fn literal(input: &str) -> Result<(&str, Literal), InterpreterError> {
    alt((integer, integer))(input).map_err(|x| InterpreterError::ParseError(x))
}

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn integer() {
        assert_eq!(super::integer("381283"), Ok(("",
            super::Literal::Integer(381283) 
        )));
    }
}