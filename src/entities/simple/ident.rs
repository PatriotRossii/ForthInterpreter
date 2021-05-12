use crate::parser::{Parse, Rule};
use crate::{
    entities::simple::literal::{Literal, Pointer},
    ExecuteExt, Result,
};

#[derive(Debug, Clone)]
pub struct Ident {
    name: String,
}

impl Ident {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Parse for Ident {
    fn parse(pair: pest::iterators::Pair<Rule>) -> Self {
        Self {
            name: pair.as_str().to_string(),
        }
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl ExecuteExt for Ident {
    fn execute(&self, interpreter: &mut crate::ForthInterpreter) -> Result<()> {
        let name = &self.name;

        if interpreter.contains_variable(name) {
            interpreter.push(Literal::Pointer(Pointer {
                address: interpreter.get_variable_id(name).unwrap(),
                offset: 0,
            }));
        }

        if interpreter.constants.contains_key(name) {
            interpreter.push(interpreter.constants.get(name).unwrap().clone());
        }

        if interpreter.native_words.contains_key(name) {
            let word = interpreter.native_words.get(name).unwrap();
            word(interpreter)?;
        }

        if interpreter.user_words.contains_key(name) {
            let word = interpreter.user_words.get(name).unwrap().clone();
            for element in word {
                element.execute(interpreter)?;
            }
        }

        Ok(())
    }
}
