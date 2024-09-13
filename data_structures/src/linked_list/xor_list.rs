#![allow(unused)]

#[derive(Debug)]
pub struct Node<T: std::fmt::Debug> {
    value: T,
    both: Option<*mut Node<T>>, // both means [NEXT xor PREVIOUS]
}
impl<T: std::fmt::Debug> Node<T> {
    fn new(value: T) -> Node<T> {
        Self { value, both: None }
    }
}

#[derive(Debug)]
pub struct Xor<T: std::fmt::Debug> {
    size: u32,
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}
impl<T: std::fmt::Debug> Xor<T> {
    pub fn new() -> Self {
        Xor {
            size: 0,
            head: None,
            tail: None,
        }
    }
}
