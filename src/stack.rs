pub struct Stack<T> {
        stack: Vec<T>
}

impl<T> Stack<T> {
        fn push(&mut self, value: T) {
                self.stack.push(value);
        }
        fn pop(&mut self) -> Option<T> {
                self.stack.pop()
        }
}
