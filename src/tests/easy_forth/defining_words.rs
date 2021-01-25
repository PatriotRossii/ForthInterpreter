#[cfg(test)]
mod defining_words_chapter {
    use crate::{ForthInterpreter, Literal};

    #[test]
    fn test() {
        let mut interpreter = ForthInterpreter::new();

        interpreter
            .execute(": foo 100 + ;\n1000 foo\nfoo foo foo")
            .unwrap();
        assert_eq!(
            interpreter.get_last_literal().unwrap(),
            &Literal::Integer(1400)
        );
    }
}
