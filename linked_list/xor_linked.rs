use core::fmt::Debug;
#[allow(unused)]
pub struct XorNode<T: Debug> {
    value: T,
    both: Option<Box<XorNode<T>>>, // this is a node for the XORed linked list
}
