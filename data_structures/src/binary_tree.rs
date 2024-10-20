// building binary tree using linked list implementation using ITERATION over RECURSION because
// ITERATION better.
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
    data: T,
    left_child: Option<*mut Node<T>>,
    right_child: Option<*mut Node<T>>,
}
impl<T: Debug> Node<T> {
    fn new(data: T) -> Node<T> {
        Self {
            data,
            left_child: None,
            right_child: None,
        }
    }
}

pub struct BSTree<T: Debug> {
    size: u32,
    head: Option<*mut Node<T>>,
}
#[allow(unused)]
impl<T> BSTree<T>
where
    T: Debug + PartialOrd + Copy,
{
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }
    pub fn insert(&mut self, val: T) {
        if self.size == 0 {
            let head_node = Box::new(Node::new(val));
            let head_node_ptr = Box::into_raw(head_node);
            self.head = Some(head_node_ptr);
        } else {
            let new_node = Box::new(Node::new(val));
            let new_node_ptr = Box::into_raw(new_node);
            let mut parent_ptr = None;
            let mut current_ptr = self.head;

            unsafe {
                while let Some(ptr) = current_ptr {
                    let current_node = &*ptr;
                    if val > current_node.data {
                        parent_ptr = Some(ptr);
                        current_ptr = current_node.right_child;
                    } else if val < current_node.data {
                        parent_ptr = Some(ptr);
                        current_ptr = current_node.left_child;
                    } else {
                        return; // Value already exists
                    }
                }

                if let Some(parent) = parent_ptr {
                    let parent_node = &mut *parent;
                    if val > parent_node.data {
                        parent_node.right_child = Some(new_node_ptr);
                    } else {
                        parent_node.left_child = Some(new_node_ptr);
                    }
                }
            }
        }
        self.size += 1;
    }
    fn delete(&mut self, val: T) {}
    fn find(&self, val: T) -> Option<*mut Node<T>> {
        let mut current_ptr = self.head;
        let mut parent_ptr: Option<*mut Node<T>> = None;
        unsafe {
            while let Some(ptr) = current_ptr {
                let current_node = &*ptr;
                if val > current_node.data {}
            }
        }
        current_ptr
    }
    fn print_tree(&self) {}
}

impl<T> Default for BSTree<T>
where
    T: Debug + PartialOrd + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
