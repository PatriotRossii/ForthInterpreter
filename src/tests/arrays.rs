#[cfg(test)]
mod array_tests {
    use crate::ForthInterpreter;
    use crate::entities::simple::literal::Literal;

    #[test]
    fn test_create() {
        let mut interpreter = ForthInterpreter::new();
        interpreter.execute_line("variable numbers");
        interpreter.execute_line("3 cells allot");

        let dump = interpreter.get_vars_dump();
        
        assert_eq!(dump[0].name, "numbers");
        match &dump[0].value {
            Some(Literal::Array(arr)) => {
                assert_eq!(arr.capacity(), 4);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_access() {
        let mut interpreter = ForthInterpreter::new();
        interpreter.execute_line("variable numbers");
        interpreter.execute_line("3 cells allot");

        interpreter.execute_line("10 numbers 0 cells + !");
        interpreter.execute_line("20 numbers 1 cells + !");
        interpreter.execute_line("30 numbers 2 cells + !");
        interpreter.execute_line("40 numbers 3 cells + !");
        interpreter.execute_line("50 numbers 3 cells + !");
        interpreter.execute_line("100 numbers 4 cells + !").unwrap_err();
        
        let dump = interpreter.get_vars_dump();
        match &dump[0].value {
            Some(Literal::Array(arr)) => {
                assert_eq!(arr[0], Literal::Integer(10));
                assert_eq!(arr[1], Literal::Integer(20));
                assert_eq!(arr[2], Literal::Integer(30));
                assert_eq!(arr[3], Literal::Integer(50));
            },
            _ => assert!(false)
        }
        assert!(interpreter.get_stack_dump().is_empty());

        interpreter.execute_line("numbers 2 cells + @");
        assert_eq!(interpreter.get_last_literal().unwrap(), &Literal::Integer(30));

        interpreter.execute_line("numbers 4 cells + @").unwrap_err();
    }
}