use cpython::{PyList, PyObject, Python, PythonObject, ToPyObject};

#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T: Clone> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
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

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl<T: ToPyObject + Clone> ToPyObject for Stack<T> {
    type ObjectType = PyList;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        PyList::new(
            py,
            self.stack
                .clone()
                .into_iter()
                .map(|e| e.to_py_object(py).into_object())
                .collect::<Vec<PyObject>>()
                .as_slice(),
        )
    }
}

impl<T: Clone> From<Vec<T>> for Stack<T> {
    fn from(value: Vec<T>) -> Stack<T> {
        Self { stack: value }
    }
}
