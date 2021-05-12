#[cfg(test)]
mod arrays_chapter {
    use crate::{ForthInterpreter, Literal};

    #[test]
    fn arrays() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute("variable numbers").unwrap();
        interpreter.execute("3 cells allot").unwrap();

        for i in 0..=3 {
            interpreter
                .execute(&format!("{} numbers {} cells + !", i * 10, i))
                .unwrap();
            interpreter
                .execute(&format!("numbers {} cells + @", i))
                .unwrap();
            assert_eq!(
                interpreter.get_unary_operand().unwrap(),
                Literal::Integer(i * 10)
            );
        }

        interpreter.execute(": number cells number + ;").unwrap();
        for i in 0..=3 {
            interpreter
                .execute(&format!("{} {} number !", i * 10, i))
                .unwrap();
            interpreter.execute(&format!("{} number @", i)).unwrap()
        }
    }
}
