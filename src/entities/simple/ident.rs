use crate::parser::Parser;

pub struct Ident {
	name: String,
}

impl Parser for Ident {
	fn parse(pair: pest::iterators::Pair<parser::Rule>) -> Self {
		Ident {
			name: ident.as_str().into_string(),
		}
	}
}
