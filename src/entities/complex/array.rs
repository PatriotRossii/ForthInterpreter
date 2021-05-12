use crate::{errors::ForthError, Literal, Result};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Array {
    storage: Vec<Literal>,
    size: usize,
    capacity: usize,
}

impl Array {
    #[inline]
    pub fn new(size: usize) -> Self {
        Self {
            storage: vec![Literal::Integer(0); size],
            capacity: size,
            size: 0,
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&Literal> {
        self.storage.get(index)
    }

    #[inline]
    pub fn push(&mut self, value: Literal) -> Result<()> {
        if self.size == self.capacity {
            Err(ForthError::IndexOutOfBound)
        } else {
            self.storage.push(value);
            self.size += 1;
            Ok(())
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize, value: Literal) -> Result<()> {
        if index < self.capacity {
            self.storage[index] = value;
            Ok(())
        } else {
            Err(ForthError::IndexOutOfBound)
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Index<usize> for Array {
    type Output = Literal;
    fn index(&self, index: usize) -> &Self::Output {
        &self.storage[index]
    }
}

impl IndexMut<usize> for Array {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.storage[index]
    }
}
