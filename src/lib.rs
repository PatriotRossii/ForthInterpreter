pub mod parser;
#[derive(Debug)]
pub enum InterpreterError<'a> {
    ParseError(nom::Err<nom::error::Error<&'a str>>)
}