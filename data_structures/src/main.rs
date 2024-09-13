use data_structures::binary_tree::BSTree;
use data_structures::hashing::hashing_theory::search_query_loops;
use data_structures::linked_list::LinkedList;

fn main() {
    println!("Hello, world!");
    let mut linked: LinkedList<i32> = LinkedList::new();
    println!("{:?}", linked);
    linked.print_list();
    linked.print_list_reversed();

    // change node operation
    // let iwantthisnode = linked.get_node(9);
    // println!("{:?}", iwantthisnode);
    // linked.change_node_value(9, 69);
    // linked.print_list();

    // deleting head operation
    linked.insert_at_head(10);
    linked.delete_at_head();
    linked.print_list();

    // deleting tail operation
    linked.insert_at_tail(20);
    linked.delete_at_tail();
    linked.print_list();

    // insert at ith position
    linked.insert_at_head(10);
    linked.insert_at_tail(20);
    linked.insert_at_tail(30);
    linked.insert_at_ith(3, 40);
    linked.print_list();
    for _ in 0..4 {
        linked.delete_at_head();
    }
    linked.print_list();
    linked.insert_at_head(10);
    linked.insert_at_tail(20);
    linked.insert_at_tail(30);
    linked.insert_at_tail(40);
    linked.insert_at_tail(50);
    linked.print_list();
    linked.delete_at_ith(3);
    linked.print_list();
    linked.insert_at_ith(3, 69);
    linked.print_list();

    for _ in 0..5 {
        linked.delete_at_tail();
    }
    linked.print_list();
    linked.insert_at_tail(10);
    linked.insert_at_tail(20);
    linked.insert_at_tail(50);
    linked.print_list();
    for i in 1..=3 {
        println!("{:?}", linked.get_node(i));
        println!("{:p}", linked.get_node(i));
    }
    linked.print_list();

    use data_structures::stacks::Stack;
    println!("STACK OPERATIONS");
    let mut mystack = Stack::new();
    mystack.push(10);
    mystack.push(20);
    mystack.push(30);
    mystack.push(40);
    mystack.pop();
    mystack.print_stack();
    mystack.peak();
    mystack.get_size();

    use data_structures::queue::Queue;
    println!("QUEUE OPERATIONS");
    let mut myqueue = Queue::new();
    myqueue.enqueue(10);
    myqueue.enqueue(20);
    myqueue.enqueue(30);
    myqueue.dequeue();
    myqueue.enqueue(40);
    myqueue.print_queue();
    myqueue.get_size();

    // BINARY TREE OPERATIONS
    println!("BINARY TREE OPERATIONS");
    let mut a = BSTree::new();
    a.insert(20);
    a.insert(10);
    a.insert(15);

    search_query_loops(&[1, 2, 1, 3, 2], &[1, 3, 4, 2, 10]);
}
