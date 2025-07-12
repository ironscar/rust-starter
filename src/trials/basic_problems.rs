// number palindrome (easier)
pub fn basic_problem_1() {
    println!("basic problem 1 - Is a number palindrome easier?");

    // easy method: by converting number to string
    let x = 1212;
    let x_str = x.to_string();
    let mut is_palindrome = true;
    for i in 0..(x_str.len()/2) {
        let x_chr1 = x_str.chars().nth(i);
        let x_chr2 = x_str.chars().nth(x_str.len() - 1 - i);
        if x_chr1.is_none() || x_chr2.is_none() {
            is_palindrome = false;
            break;
        }
        if x_chr1.unwrap() != x_chr2.unwrap() {
            is_palindrome = false;
            break;
        }
    }
    if is_palindrome {
        println!("{} is palindrome", x);
    } else {
        println!("{} is not a palindrome", x);
    }
}

// number palindrome (harder)
pub fn basic_problem_2() {
    println!("basic problem 2 - Is a number palindrome harder?");

    // hard method: by not converting number to string
    let x = 121;
    let mut x_copy = x;
    let mut x_vec: Vec<u16> = Vec::new();
    while x_copy > 0 {
        let y = x_copy % 10;
        x_vec.push(y);
        x_copy /= 10;
    }
    let mut is_palindrome = true;
    for i in 0..(x_vec.len()/2) {
        let x1 = x_vec[i];
        let x2 = x_vec[x_vec.len()-1-i];
        if x1 != x2 {
            is_palindrome = false;
            break;
        }
    }
    if is_palindrome {
        println!("{} is palindrome", x);
    } else {
        println!("{} is not a palindrome", x);
    }
}

// roman numeral to integer
pub fn basic_problem_3() {
    println!("basic problem 3 - Convert roman numeral below 400 to integer");

    let x = "XLIV";
    let xlen = x.len();
    let mut actual_num = 0;
    let mut index = 0;
    while index < xlen {
        let xchr1 = x.chars().nth(index);
        let xchr2 = x.chars().nth(index+1);
        if xchr1.unwrap() == 'I' {
            if xchr2.is_none() {
                actual_num += 1;
                break;
            } else if xchr2.unwrap() == 'V' {
                actual_num += 4;
            } else if xchr2.unwrap() == 'X' {
                actual_num += 9;
            }
            index += 2;
            continue;
        } else if xchr1.unwrap() == 'V' {
            actual_num += 5;
        } else if xchr1.unwrap() == 'X' {
            if xchr2.is_none() {
                actual_num += 10;
                break;
            } else if xchr2.unwrap() == 'L' {
                actual_num += 40;
            } else if xchr2.unwrap() == 'C' {
                actual_num += 90;
            }
            index += 2;
            continue;
        } else if xchr1.unwrap() == 'L' {
            actual_num += 50;
        } else if xchr1.unwrap() == 'C' {
            actual_num += 100;
        }
        index += 1;
    }
    println!("{} = {}", x, actual_num);
}

// longest common prefix
pub fn basic_problem_4() {
    println!("basic problem 4 - Longest common prefix");

    let x = ["racedog", "racecar", "car"];
    let xlen = x.len();
    if xlen == 0 {
        println!("input list is empty");
        return;
    }
    let mut index = 0;
    let mut prefix = String::new();
    'outer: loop {
        let x_char = x.get(0).unwrap().chars().nth(index);
        let mut is_common = true;
        for i in 1..xlen {
            let curr_char = x.get(i).unwrap().chars().nth(index);
            if x_char.is_none() || curr_char.is_none() {
                is_common = false;
                break 'outer;
            }
            if x_char.unwrap() != curr_char.unwrap() {
                is_common = false;
                break 'outer;
            }
        }
        if is_common {
            prefix += x_char.unwrap().to_string().as_str();
            index += 1;
        } else {
            break 'outer;
        }
    }
    if prefix.is_empty() {
        println!("There is no common prefix");
    } else {
        println!("Longest common prefix = {}", prefix);
    }
}

// merge two sorted lists
pub fn basic_problem_5() {
    println!("basic problem 5 - Merge two sorted lists");

    let x1 = [1,2,5,6,9,13];
    let x2 = [2,3,4,7,11];
    let mut i1 = 0;
    let mut i2 = 0;
    let mut xres: Vec<i32> = Vec::new();
    while i1 < x1.len() && i2 < x2.len() {
        let last_elem = if xres.is_empty() { None } else { xres.get(xres.len() - 1) };
        if i2 >= x2.len() || x1[i1] <= x2[i2] {
            // if i2 is over length, i1 won't be as while condition prevents that
            if last_elem.is_none() || *last_elem.unwrap() != x1[i1] {
                xres.push(x1[i1]);
            }
            i1 += 1;
        } else if i1 >= x1.len() || x2[i2] <= x1[i1] {
            // if i1 is over length, i2 won't be as while condition prevents that
            if last_elem.is_none() || *last_elem.unwrap() != x2[i2] {
                xres.push(x2[i2]);
            }
            i2 += 1;
        }
    }
    println!("xres deduplicated = {:?}", xres);
}

// valid paranthesis
pub fn basic_problem_6() {
    println!("basic problem 6 - Valid paranthesis");

    let str = String::from("((){}[{})]){}");
    let chars: Vec<char> = str.chars().collect();
    let mut new_chars: Vec<char> = Vec::new();
    let mut result = true;

    for c in chars {
        if c == '(' || c == '{' || c == '[' {
            new_chars.push(c);
        } else if c == ')' || c == '}' || c == ']' {
            if new_chars.len() == 0 {
               result = false;
               break;
            }
            let last_char = new_chars.get(new_chars.len() - 1).unwrap();
            if (*last_char == '(' && c == ')') ||
                (*last_char == '{' && c == '}') ||
                (*last_char == '[' && c == ']')
            {
                new_chars.pop();
            } else {
                new_chars.push(c);
            }
        }
    }

    if new_chars.is_empty() {
        println!("Valid paranthesis");
    } else {
        println!("Invalid paranthesis");
    }
}
