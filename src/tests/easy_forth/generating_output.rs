#[cfg(test)]
mod generating_output_chapter {
    use crate::ForthInterpreter;

    #[test]
    fn test() {
        let mut interpreter = ForthInterpreter::new();

        interpreter.execute("1 . 2 . 3 . 4 5 6 . . .").unwrap(); // Should output: "1 2 3 6 5 4"
        interpreter
            .execute("33 119 111 87 emit emit emit emit")
            .unwrap(); // Should output: "Wow!"
        interpreter.execute("cr 100 . cr 200 . cr 300 .").unwrap(); // Should output: "\n100\n200\n300"
        interpreter
            .execute(r#": say_hello ." Hello there!" ;\n say-hello"#)
            .unwrap_err(); // Should output: "Hello there!"
        interpreter
            .execute(r#": print_stack_top  cr dup ." The top of the stack is " .\cr ." which looks like '" dup emit ." ' in ascii  " ;\n48 print-stack-top"#)
            .unwrap(); //Should output: "\nThe top of the stack is 48\nwhich looks like '0' in ascii  "
        todo!("CHECK OUTPUT MANUALLY, LOL!");
    }
}
