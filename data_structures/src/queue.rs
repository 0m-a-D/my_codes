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
pub struct Queue<T: std::fmt::Debug> {
    size: u32,
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}
impl<T: std::fmt::Debug> Queue<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }
    // here enqueue is "insert_at_tail" of linked list
    pub fn enqueue(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));
        let raw_ptr = Box::into_raw(node);
        unsafe {
            if self.size == 0 {
                self.head = Some(raw_ptr);
                self.tail = Some(raw_ptr);
            } else {
                let old_tail = self.tail.unwrap();
                self.tail = Some(raw_ptr);
                (*self.tail.unwrap()).prev = Some(old_tail);
                (*old_tail).next = self.tail;
            }
        }
        self.size += 1;
    }
    // here dequeue is "delete_at_head" of linked list
    pub fn dequeue(&mut self) {
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
    pub fn print_queue(&self) {
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
    pub fn get_size(&self) {
        println!("{}", self.size);
    }
}
