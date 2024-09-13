// built using linked list implementation...
#[derive(Debug)]
pub struct Node<T: std::fmt::Debug> {
    value: T,
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
}
impl<T: std::fmt::Debug> Node<T> {
    fn new(value: T) -> Node<T> {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct Stack<T: std::fmt::Debug> {
    size: u32,
    pub head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}
impl<T: std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }
    // push is nothing but "insert_at_head" of linked list
    pub fn push(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));
        let raw_ptr = Box::into_raw(node);
        unsafe {
            if self.size == 0 {
                self.head = Some(raw_ptr);
                self.tail = Some(raw_ptr);
            } else {
                let old_head = self.head.unwrap();
                self.head = Some(raw_ptr);
                (*self.head.unwrap()).next = Some(old_head);
                (*old_head).prev = self.head;
            }
        }
        self.size += 1;
    }
    // pop is nothing but "delete_at_head" of linked list
    pub fn pop(&mut self) {
        let old_head = self.head;
        unsafe {
            if let Some(node) = old_head {
                self.head = (*node).next;
                if let Some(new_head) = self.head {
                    (*new_head).prev = None;
                } else {
                    self.tail = None;
                }
                let _ = Box::from_raw(node);
            }
        }
        self.size -= 1;
    }
    pub fn peak(&self) {
        unsafe {
            println!("{:?}", (*self.head.unwrap()).value);
        }
    }
    pub fn get_size(&self) {
        println!("{}", self.size);
    }
    pub fn print_stack(&self) {
        let mut counter = self.size;
        let mut head = self.head;
        unsafe {
            while counter > 0 {
                if let Some(node) = head {
                    print!("{:?} -> ", (*node).value);
                    head = (*node).next;
                    counter -= 1;
                }
            }
            println!();
        }
    }
}
impl<T: std::fmt::Debug> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
