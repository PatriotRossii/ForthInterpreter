#[cfg(test)]
mod stack_manipulating_chapter {
    use crate::{ForthInterpreter, Stack};

    #[test]
    fn test() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute("1 2 3 dup").unwrap();
        assert_eq!(
            interpreter.get_stack_dump(),
            &Stack::from(vec![1.into(), 2.into(), 3.into(), 3.into()])
        );
        interpreter.clear_state();

        interpreter.execute("1 2 3 drop").unwrap();
        assert_eq!(
            interpreter.get_stack_dump(),
            &Stack::from(vec![1.into(), 2.into()])
        );
        interpreter.clear_state();

        interpreter.execute("1 2 3 4 swap").unwrap();
        assert_eq!(
            interpreter.get_stack_dump(),
            &Stack::from(vec![1.into(), 2.into(), 4.into(), 3.into()])
        );
        interpreter.clear_state();

        interpreter.execute("1 2 3 over").unwrap();
        assert_eq!(
            interpreter.get_stack_dump(),
            &Stack::from(vec![1.into(), 2.into(), 3.into(), 2.into()])
        );
        interpreter.clear_state();

        interpreter.execute("1 2 3 rot").unwrap();
        assert_eq!(
            interpreter.get_stack_dump(),
            &Stack::from(vec![2.into(), 3.into(), 1.into()])
        );
        interpreter.clear_state();
    }
}
