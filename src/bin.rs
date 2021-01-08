use forth_interpreter::ForthInterpreter;

fn main() {
    let mut interpreter = ForthInterpreter::new();
    
    interpreter.execute_line("1 2 3 *");
    println!("{}", interpreter.get_stack_dump());
}