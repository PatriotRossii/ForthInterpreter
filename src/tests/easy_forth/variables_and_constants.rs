#[cfg(test)]
mod variables_and_constants_chapter {
    use crate::{ForthInterpreter, Literal, Pointer};

    #[test]
    fn variables() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute("variable balance").unwrap();
        interpreter.execute("balance").unwrap();
        assert_eq!(
            interpreter.get_unary_operand().unwrap(),
            Literal::Pointer(Pointer::new(0, 0))
        );

        interpreter.execute("123 balance !").unwrap();
        interpreter.execute("balance @").unwrap();
        assert_eq!(
            interpreter.get_unary_operand().unwrap(),
            Literal::Integer(123)
        );

        interpreter.execute("50 balance +!").unwrap();
        interpreter.execute("balance @").unwrap();
        assert_eq!(
            interpreter.get_unary_operand().unwrap(),
            Literal::Integer(123 + 50)
        );
    }

    #[test]
    fn constants() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute("42 constant answer").unwrap();
        interpreter.execute("2 answer *").unwrap();
        assert_eq!(
            interpreter.get_unary_operand().unwrap(),
            Literal::Integer(42 * 2)
        );
    }
}
