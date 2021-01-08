#[derive(Debug)]
pub struct Stack<T> {
        stack: Vec<T>
}

impl<T: Clone> Stack<T> {
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

	pub fn length(&self) -> usize {
		self.stack.len()
	}

	pub(crate) fn swap(&mut self, a: usize, b: usize) {
		self.stack.swap(a, b)
	}

	pub(crate) fn get(&self, a: usize) -> &T {
		&self.stack[a]
	}

	pub(crate) fn remove(&mut self, a: usize) -> T {
		self.stack.remove(a)
	}
}
