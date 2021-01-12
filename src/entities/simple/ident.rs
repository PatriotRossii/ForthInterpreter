use crate::parser::*;
use crate::{ExecuteExt, entities::{simple::literal::Literal}, Result};

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
		let variable = interpreter.variables.get(name);

		if let Some(e) = variable {
			interpreter.push((e as *const Option<Literal> as i64).into());
		}

		let r#const = interpreter.constants.get(name);
		if let Some(e) = r#const {
			interpreter.push(e.clone());
		}

		let word = interpreter.native_words.get(name);
		if let Some(e) = word {
			e(interpreter)?;
		}

		let word = interpreter.user_words.get_mut(name);
		if let Some(e) = word {
			e.clone().execute(interpreter)?;
		}

		Ok(())
	}
}
