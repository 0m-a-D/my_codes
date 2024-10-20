// Unsafe doubly linked list implementation [use box to free created nodes after program ends]
// scrap out entire implementation to USE smart pointers...
mod xor_list;
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
pub struct LinkedList<T: std::fmt::Debug> {
    size: u32,
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}
impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }
    pub fn insert_at_head(&mut self, obj: T) {
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
    pub fn insert_at_tail(&mut self, obj: T) {
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
    /// USE THIS FOR self.size > INDEX > 1...else use insert_at_head() / insert_at_tail()
    pub fn insert_at_ith(&mut self, index: u32, obj: T) {
        if index > self.size {
            panic!("index out of bounds! should be between 0 and {}", self.size);
        }
        let new_node = Box::new(Node::new(obj));
        let raw_ptr = Box::into_raw(new_node);
        let node_at_index = self.get_node(index);

        unsafe {
            (*raw_ptr).next = Some(&mut *node_at_index);
            let previous_node = (node_at_index).prev.unwrap();
            (*previous_node).next = Some(&mut *raw_ptr);
            (*raw_ptr).prev = Some(&mut *previous_node);
        }
        self.size += 1;
    }
    pub fn delete_at_head(&mut self) {
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
    pub fn delete_at_tail(&mut self) {
        let old_tail = self.tail;
        unsafe {
            if let Some(node) = old_tail {
                self.tail = (*node).prev;
                if let Some(new_tail) = self.tail {
                    (*new_tail).next = None;
                } else {
                    self.head = None;
                }
                let _ = Box::from_raw(node);
            }
        }
        self.size -= 1;
    }
    /// USE THIS FOR self.size > INDEX > 1...else use delete_at_head() / delete_at_tail()
    pub fn delete_at_ith(&mut self, index: u32) {
        if index > self.size {
            panic!("index out of bounds! should be between 0 and {}", self.size);
        }
        let deleted_node = self.get_node(index);
        let prev = (deleted_node).prev;
        let next = (deleted_node).next;
        unsafe {
            (*prev.unwrap()).next = next;
            (*next.unwrap()).prev = prev;
            let _ = Box::from_raw(deleted_node);
        }
        self.size -= 1;
    }
    pub fn get_node(&mut self, mut index: u32) -> &mut Node<T> {
        if index > self.size {
            panic!(
                "index out of bounds! should be between 0 and length of this list: {}",
                self.size
            );
        }
        if index < self.size / 2 {
            unsafe {
                let mut head = self.head;
                let mut node_ptr = head;
                while index > 1 {
                    node_ptr = (*head.unwrap()).next;
                    head = node_ptr;
                    index -= 1;
                }
                &mut (*node_ptr.unwrap())
            }
        } else {
            let mut tail = self.tail;
            let mut node_ptr = tail;
            unsafe {
                while self.size - index > 0 {
                    node_ptr = (*tail.unwrap()).prev;
                    tail = node_ptr;
                    index += 1;
                }
                &mut (*node_ptr.unwrap())
            }
        }
    }
    pub fn change_node_value(&mut self, index: u32, new_val: T) {
        let node = self.get_node(index);
        (node).value = new_val;
    }
    pub fn print_list(&self) {
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
    pub fn print_list_reversed(&self) {
        let mut counter = self.size;
        let mut tail = self.tail;
        unsafe {
            while counter > 0 {
                if let Some(node) = tail {
                    print!("{:?} -> ", (*node).value);
                    tail = (*node).prev;
                    counter -= 1;
                }
            }
            println!();
        }
    }
}
impl<T: std::fmt::Debug> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
