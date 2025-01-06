// Unsafe doubly linked list implementation [use box to free created nodes after program ends]
// scrap out entire implementation to USE smart pointers...
mod xor_list;
#[derive(Debug, Clone, Copy)]
pub struct Node<T: std::fmt::Debug>
where
    T: Copy + Clone,
{
    pub value: T,
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
}
impl<T: std::fmt::Debug> Node<T>
where
    T: Copy + Clone,
{
    fn new(value: T) -> Node<T> {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LinkedList<T: std::fmt::Debug>
where
    T: Copy + Clone,
{
    size: u32,
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}
impl<T: std::fmt::Debug> LinkedList<T>
where
    T: Copy + Clone,
{
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
    /// USE THIS FOR INDEX ∈ [0, self.size-1]...else use insert_at_head() / insert_at_tail()
    pub fn insert_at_ith(&mut self, index: u32, obj: T) {
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
    /// USE THIS FOR INDEX ∈ [0, self.size-1]...else use delete_at_head() / delete_at_tail()
    pub fn delete_at_ith(&mut self, index: u32) {
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

    // FIXME: indexing issue in get_node()
    pub fn get_node(&mut self, index: u32) -> &mut Node<T> {
        if index >= self.size {
            panic!(
                "index out of bounds! should be between 0 and length of this list: {}",
                self.size
            );
        }
        if index < self.size / 2 {
            unsafe {
                let mut current = self.head;
                let mut count = 0;

                while count < index {
                    current = (*current.unwrap()).next;
                    count += 1;
                }

                &mut (*current.unwrap())
            }
        } else {
            unsafe {
                let mut current = self.tail;
                let mut count = self.size - 1;

                while count > index {
                    current = (*current.unwrap()).prev;
                    count -= 1;
                }

                &mut (*current.unwrap())
            }
        }
    }
    pub fn change_node_value(&mut self, index: u32, new_val: T) {
        let node = self.get_node(index);
        (node).value = new_val;
    }
    pub fn print_list(&self) -> Vec<T> {
        let mut counter = self.size;
        let mut head = self.head;
        let mut nodes = Vec::with_capacity(self.size as usize);
        unsafe {
            while counter > 0 {
                if let Some(node) = head {
                    print!("{:?} -> ", (*node).value);
                    nodes.push((*node).value);
                    head = (*node).next;
                    counter -= 1;
                }
            }
            println!();
        }
        nodes
    }
    pub fn print_list_reversed(&self) -> Vec<T> {
        let mut counter = self.size;
        let mut tail = self.tail;
        let mut nodes = Vec::with_capacity(self.size as usize);
        unsafe {
            while counter > 0 {
                if let Some(node) = tail {
                    print!("{:?} -> ", (*node).value);
                    nodes.push((*node).value);
                    tail = (*node).prev;
                    counter -= 1;
                }
            }
            println!();
        }
        nodes
    }
    pub fn get_size(&self) -> u32 {
        self.size
    }
}
impl<T: std::fmt::Debug + std::marker::Copy> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
