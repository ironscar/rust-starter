struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct LinkedList {
    head: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Node {
        Node {value, next: None}
    }

    fn len(&self) -> usize {
        1 + match &self.next {
            Some(node) => node.len(),
            None => 0
        }
    }

    fn print(&self) -> String {
        let mut printstr = self.value.to_string();
        let next_str = match &self.next {
            Some(n) => ",".to_string() + n.print().as_str(),
            _ => "".to_string()
        };
        printstr + next_str.as_str()
    }
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {head: None}
    }

    fn shift(&mut self, val: i32) {
        let new_node = Node{value: val, next: self.head.take()};
        self.head = Some(Box::new(new_node));
    }

    fn len(&self) -> usize {
        match &self.head {
            Some(h) => h.len(),
            None => 0
        }
    }

    fn print(&self) {
        match &self.head {
            Some(h) => println!("list = {}", h.print()),
            None => println!("None")
        };
    }
}

/*
 * TODO:
 * - PUSH
 * - POP
 * - TAIL REF
 * - CIRCULAR
 * - DOUBLY-LINKED
 */
pub fn linked_list_trial() {
    let mut linked_list = LinkedList::new();
    linked_list.shift(1);
    linked_list.shift(2);
    linked_list.shift(3);
    linked_list.print();
    println!("linked list length: {}", linked_list.len());
}
