use std::fmt;

pub fn vectors_demo() {
    println!("vectors demo begins here...");

    // instantiate empty vector with type specified
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // instantiate non-empty vector using vec! macro
    let mut v2 = vec![1, 2, 3];

    // reading values from vector and panic if out of index
    let el1 = &v1[1];
    println!("el1 = {el1}");

    // reading values from vector and handle with None if out of index
    let el2 = v2.get(2);
    match el2 {
        Some(x) => println!("el2 = {x}"),
        None => println!("el2 is None")
    }

    // iterating over vectors
    for i in &v1 {
        print!("{i},");
    }
    println!("END");

    // iterating over vector to make changes to elements using dereference(*) operator
    for i in &mut v2 {
        *i *= 10;
        print!("{i},");
    }
    println!("END");

    // Store multiple types in same vector using enums
    let mut v3: Vec<Cell> = Vec::new();
    v3.push(Cell::Int(1));
    v3.push(Cell::Text(String::from("apple")));
    for i in &v3 {
        print!("{},", i);
    }
    println!("END");
}

enum Cell {
    Int(i32),
    Text(String)
}
impl fmt::Display for Cell {
    // create a Display formatter for a multi-type enum vector
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Int(i) => write!(f, "{}", i),
            Cell::Text(s) => write!(f, "{}", s)
        }
    }
}

/*--------------------------------------------------------------------------*/

pub fn strings_demo() {
    println!("strings demo begins here...");

    // instantiate an empty string
    let mut s1 = String::new();

    // instantiate a non-empty string
    let mut s2 = String::from("apple");

    // instantiate a non-empty string from existing variable
    let x = "initial contents";
    let mut s3 = x.to_string();

    // append to strings
    s1.push_str("foo");
    s2 += "bar";
    s3.push('!');
    println!("s1 = {s1}, s2 = {s2}, s3 = {s3}");

    // format complex strings using format macro
    let s4 = format!("{s1} / {s2} / {s3}");
    println!("s4 = {}", s4);

    // slicing into strings to get parts of it
    let slice = &s4[5..8];
    println!("slice = \"{slice}\"");

    // iterating over strings as chars
    let trial_str = String::from("Зд");
    for c in trial_str.chars() {
        print!("{c}");
    }
    println!(" END");
}

/*--------------------------------------------------------------------------*/

pub fn hashmap_demo() {
    println!("hashmap demo begins here...");
}
