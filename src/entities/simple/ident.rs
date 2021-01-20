use crate::parser::*;
use crate::{
    entities::simple::literal::{Literal, Pointer},
    ExecuteExt, Result,
};

#[derive(Debug, Clone)]
pub struct Ident {
    name: String,
}

impl Parse for Ident {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        Ident {
            name: pair.as_str().to_string(),
        }
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

impl ExecuteExt for Ident {
    fn execute(&mut self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        let name = &self.name;

        if interpreter.contains_variable(name) {
            interpreter.push(Literal::Pointer(Pointer {
                address: interpreter.get_variable_id(name).unwrap(),
                offset: 0,
            }));
        }

        if interpreter.constants.contains_key(name) {
            let r#const = interpreter.constants.get(name).unwrap();
            interpreter.push(r#const.clone());
        }

        if interpreter.native_words.contains_key(name) {
            let word = interpreter.native_words.get(name).unwrap();
            word(interpreter)?;
        }

        if interpreter.user_words.contains_key(name) {
            let word = interpreter.user_words.get_mut(name).unwrap();
            word.clone().execute(interpreter)?;
        }

        Ok(())
    }
}
