use std::collections::HashMap;
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

    // create a new map and populate it
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("apple", 10);
    scores.insert("ball", 20);
    // copied is used to return Option<i32> instead of Option<&i32>
    println!("apple = {}", scores.get("apple").copied().unwrap_or(0));

    // adding key only if absent
    scores.entry("ball").or_insert(30);
    scores.entry("cat").or_insert(30);
    println!("ball = {}", scores.get("ball").copied().unwrap_or(0));
    println!("cat = {}", scores.get("cat").copied().unwrap_or(0));

    // updating value of key based on current value
    let curr_val = scores.entry("cat").or_insert(0);
    *curr_val += 1;
    let curr_val = scores.entry("dog").or_insert(0);
    *curr_val += 1;
    println!("cat = {}", scores.get("cat").copied().unwrap_or(0));
    println!("dog = {}", scores.get("dog").copied().unwrap_or(0));
}

/*--------------------------------------------------------------------------*/

/*
 * Given a list of integers, find median and mode
 */
pub fn collections_exercises_1() {
    println!("collections exercises begins here...");
    let mut v = vec![1, 20, 3, 5, 45, 23, 5, 98, 7];
    v.sort();
    let median = v.get(v.len() / 2).unwrap();

    let mut mode_map: HashMap<i32, i32> = HashMap::new();
    let mut max_val = 0;
    let mut max_elem = 0;
    for val in &v {
        let count = mode_map.entry(*val).or_insert(0);
        *count += 1;
        if *count > max_val {
            max_val = *count;
            max_elem = *val;
        }
    }

    println!("Result 1: mode = {max_elem} and median = {median}");
}

/*
 * Given a string
 * if it starts with consonant, take it to end of word and add ay to end of word
 * if it starts with consonant, add hay to end of word
 */
pub fn collections_exercises_2() {
    let str = String::from("There is a brown fox");
    let mut words_iter = str.split_whitespace();
    let mut word = words_iter.next();
    let mut new_str = String::new();

    loop {
        let actual_word = word.unwrap();
        let chars: Vec<char> = actual_word.chars().collect();
        let mut new_chars = chars.clone();

        let first_char = chars.first().unwrap();
        if *first_char == 'a' || *first_char == 'e' || *first_char == 'i' ||
            *first_char == 'o' || *first_char == 'u'
        {
            new_chars.push('h');
            new_chars.push('a');
            new_chars.push('y');
        } else {
            new_chars.remove(0);
            new_chars.push(*first_char);
            new_chars.push('a');
            new_chars.push('y');
        }
        let new_word: String = new_chars.iter().collect();
        new_str.push_str(&new_word);

        word = words_iter.next();
        if word != None {
            new_str.push(' ');
        } else {
            break;
        }
    }

    println!("Result 2: = \"{new_str}\"");
}

/*
 * allow adding people to departments using a function
 * allow department changes using same function
 * get all people within a specific department using a function
 * one person can be only in one department at a time
 */
pub fn collections_exercises_3() {
    let mut ppl_map: HashMap<String, String> = HashMap::new();

    add_person_to_dept(&mut ppl_map, "Adam", "SMS");
    add_person_to_dept(&mut ppl_map, "Eve", "SCS");
    add_person_to_dept(&mut ppl_map, "Jacob", "FPS");
    add_person_to_dept(&mut ppl_map, "Adam", "SCS");

    get_department(&ppl_map, "SMS");
    get_department(&ppl_map, "SCS");
    get_department(&ppl_map, "FPS");
}

fn add_person_to_dept(ppl_map: &mut HashMap<String, String>, person_name: &str, dept: &str) {
    ppl_map.insert(String::from(person_name), String::from(dept));
}

fn get_department(ppl_map: &HashMap<String, String>, dept: &str) {
    let mut ppl_list: Vec<String> = Vec::new();
    ppl_map.iter().for_each(|(k, v)| {
        if *v == dept {
            ppl_list.push(String::from(k));
        }
    });
    println!("ppl_list in {} = {:?}", dept, ppl_list);
}
