use crate::parser::*;

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
