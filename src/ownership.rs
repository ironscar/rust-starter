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

// ------------------------------------------------------------- //

pub fn slices_demo() {
    println!("slices_demo begins...");
    let mut s = String::from("hello world");
    let second_word_slice = find_second_word(&s);
    // s.clear(); // throws compile error for mutable borrow as it's already immutably borrowed
    println!("The second word is {}", second_word_slice);

}

fn find_second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // this tuple notation can be used to get index along with element by using iter().enumerate()
    let mut exists = false;
    let mut space1: usize = 0;
    let mut space2: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if !exists {
                space1 = i + 1;
                exists = true;
            } else {
                space2 = i;
                break;
            }
        }
    }
    if !exists {
        return "";
    }
    space2 = s.len();

    // string slice returns &str instead of String and contains all characters excluding end index
    // the .. range operator can skip the first number if it's 0 and last number if it's the max
    &s[space1..space2]
}
