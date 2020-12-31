#[derive(Debug)]
pub struct Stack<T> {
        stack: Vec<T>
}

impl<T> Stack<T> {
	pub fn new() -> Self {
		Self {
			stack: Vec::new()
		}
	}

        pub fn push(&mut self, value: T) {
                self.stack.push(value);
        }

        pub fn pop(&mut self) -> Option<T> {
                self.stack.pop()
        }

	pub fn last(&self) -> Option<&T> {
		self.stack.last()
	}
}
