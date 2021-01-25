#[cfg(test)]
mod adding_some_numbers_chapter {
    use crate::{ForthInterpreter, Literal};

    #[test]
    fn test() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute_line("123 456 +").unwrap();
        assert_eq!(
            interpreter.get_last_literal().unwrap(),
            &Literal::Integer(123 + 456)
        );
        interpreter.clear_state();

        interpreter.execute_line("5 2 + 10 *").unwrap();
        assert_eq!(
            interpreter.get_last_literal().unwrap(),
            &Literal::Integer((5 + 2) * 10)
        );
        interpreter.clear_state();

        println!("{:?}", interpreter.get_stack_dump());
        interpreter.execute_line("1 2 3 + + +").unwrap_err();
        interpreter.clear_state();
    }
}
