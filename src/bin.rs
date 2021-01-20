use forth_interpreter::ForthInterpreter;

fn main() {
    let mut interpreter = ForthInterpreter::new();

    interpreter.execute_line("variable user_var").unwrap();
    interpreter.execute_line("123 user_var !").unwrap();
    interpreter.execute_line("user_var @").unwrap();

    println!("{:?}", interpreter.get_stack_dump());
}
