/*
 * Structure for the linked list node
 */
struct LinkedListNode {
    val: i32,
    next: Option<Box<LinkedListNode>>
}

impl LinkedListNode {
    /*
     * Create new instance of node with a value
     */
    fn new(val: i32) -> Self {
        Self {val, next: None}
    }

    /*
     * Push an element to the linked list while keeping the head reference intact
     * Also returns a mutable reference of the current tail after push
     */
    fn push(&mut self, val: i32) -> &mut Box<LinkedListNode> {
        match self.next {
            Some(ref mut next_node) => {
                next_node.push(val)
            },
            None => {
                self.next = Some(Box::new(LinkedListNode::new(val)));
                self.next.as_mut().unwrap()
            }
        }
    }

    /*
     * Pop tail element from the linked list while keeping the head reference intact
     */
    fn pop(&mut self) -> Option<Box<LinkedListNode>> {
        if self.next.is_some() {
            let next_node = self.next.as_ref().unwrap();
            if next_node.next.is_some() {
                self.next.as_mut().unwrap().pop()
            } else {
                // technically a cloned value of the pop
                let actual_val = next_node.val;
                self.next = None;
                Some(Box::new(LinkedListNode::new(actual_val)))
            }
        } else {
            // cannot pop if only one element
            None
        }
    }

    /*
     * Returns the length of the linked list while keeping the head reference intact
     */
    fn length(&self) -> usize {
        1 + match self.next {
            Some(ref next_node) => next_node.length(),
            None => {0}
        }
    }

    /*
     * Returns a string representation of the linked list while keeping the head reference intact
     */
    fn print(&self) -> String {
        let mut display_str = self.val.to_string();
        let next_str = match self.next {
            Some(ref next_node) => next_node.print(),
            None => String::from("END")
        };
        display_str + "->" + next_str.as_str()
    }
}

/*
 * TODO:
 * - PUSH (DONE)
 * - POP (DONE)
 * - TAIL REF (PARTIALLY - but currently kind of useless)
 * - DOUBLY-LINKED
 * - CIRCULAR
 */
pub fn linked_list_trial() {
    // pushing and length
    let mut head = LinkedListNode::new(1);
    head.push(2);
    println!("list size = {}, vals = {}", head.length(), head.print());

    // get tail reference from push
    let mut tail = head.push(3);
    println!("tail = {}", tail.val);
    println!("list size = {}, vals = {}", head.length(), head.print());

    // popping while preserving preference
    let popped_node = head.pop();
    println!("Popped element = {}",
         if popped_node.is_some() {
            popped_node.unwrap().val.to_string()
        } else { String::from("None") }
    );
    println!("list size = {}, vals = {}", head.length(), head.print());
}
