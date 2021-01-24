use crate::Literal;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Option<Literal>,
}

impl Variable {
    pub fn get_mut(&mut self) -> Option<&mut Literal> {
        (&mut self.value).as_mut()
    }
}
