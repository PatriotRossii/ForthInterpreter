#[cfg(test)]
mod defining_words_chapter {
    use crate::{ForthInterpreter, Literal};

    #[test]
    fn booleans() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute("3 4 =").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(0));

        interpreter.execute("5 5 =").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(-1));


        interpreter.execute("3 4 <").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(-1));

        interpreter.execute("3 4 >").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(0));

        interpreter.execute("3 4 < 20 30 < and").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(-1));

        interpreter.execute("3 4 < 20 30 > or").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(-1));

        interpreter.execute("3 4 < invert").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(0));
    }

    #[test]
    fn if_then() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute(r#": buzz? 5 mod 0 = if 1 then ;"#).unwrap();
        
        interpreter.execute("3 buzz?").unwrap();
        interpreter.get_unary_operand().unwrap_err();

        interpreter.execute("4 buzz?").unwrap();
        interpreter.get_unary_operand().unwrap_err();

        interpreter.execute("5 buzz?").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(1));
    }

    fn if_else_then() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute(": is-it-zero? 0 = if 1 else 0 then ;").unwrap();
        
        interpreter.execute("0 is-it-zero?").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(1));

        interpreter.execute("1 is-it-zero?").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(0));

        interpreter.execute("2 is-it-zero?").unwrap();
        assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(0));
    }

    fn do_loop() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute(": loop-test 10 0 do i loop ;").unwrap();
        interpreter.execute("loop-test").unwrap();

        for i in (0..10).rev() {
            assert_eq!(interpreter.get_unary_operand().unwrap(), Literal::Integer(i));
        }
    }

    fn fizz_buzz() {
        todo!("Implement FizzBuzz")
    }
}
