use std::{
    error::Error,
    fmt::{Debug, Display},
};

const MAX_SIZE: usize = 256;
pub struct CircularBuffer<T>
where
    T: Display + Debug,
{
    size: usize,
    data: Vec<T>,
}

impl<T> CircularBuffer<T>
where
    T: Display + Debug,
{
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::with_capacity(MAX_SIZE),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.size == MAX_SIZE {
            self.data.remove(0);
        } else {
            self.size += 1;
        }
        self.data.push(item);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            Some(&self.data[index])
        } else {
            None
        }
    }
}

impl<T> Iterator for CircularBuffer<T>
where
    T: Display + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.data.remove(0))
        } else {
            None
        }
    }
}

impl<T> Error for CircularBuffer<T> where T: Display + Debug {}

impl<T> Display for CircularBuffer<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CircularBuffer")
    }
}

impl<T> Debug for CircularBuffer<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CircularBuffer")
    }
}
