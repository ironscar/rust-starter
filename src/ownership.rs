pub fn ownership_demo() {
    println!("ownership demo begins...");

    let mut s1 = String::from("hello");
    let len_method = 3;

    // using values with ownership constraints
    if (len_method == 1) {
        let (s2, len) = get_length_by_ownership(s1);
        println!("The length of '{}' is {}.", s2, len);
    } else if (len_method == 2) {
        // borrowing values using references
        let len = get_length_by_reference(&s1);
        println!("The length of '{}' is {}.", s1, len);
    } else if (len_method == 3) {
        // borrowing and mutating using mutable references
        let len = get_length_mutated_by_reference(&mut s1);
        println!("The length of '{}' is {}.", s1, len);
    }
}

fn get_length_by_ownership(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn get_length_by_reference(s: &String) -> usize {
    let length = s.len();
    length
}

fn get_length_mutated_by_reference(s: &mut String) -> usize {
    s.push_str(", world");
    let length = s.len();
    length
}
