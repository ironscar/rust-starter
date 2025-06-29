pub fn basic_demo() {
    println!("Start basic control flow demo");

    let num: u8 = 3;
    // regular if else cases, no parenthesis required by default
    if num > 4 {
        println!("greater than 4");
    } else if num > 2 {
        println!("greater than 2");
    } else {
        println!("greater than 0");
    }

    // Rust doesn't have ternary, but we can use if as an expression
    // All cases must return the same type for this though
    let val = if num % 2 == 0 { "Even" } else { "Odd" };
    println!("val = {}", val);

    // Iterative constructs with loop where we can also return results from break
    // We can use return as well, but it will also end the function
    // We can also assign labels if there are multiple loops to break specifically
    let mut counter: u8 = 0;
    let loop_result = 'outer: loop {
        if counter >= 10 { break counter; }
        else {
            println!("Looping outer {counter}");
        }
        loop {
            println!("Looping inner {counter}");
            counter += 1;
            if counter % 2 == 0 { continue; }
            else if counter % 3 == 0 { break; }
            else if counter % 5 == 0 { break 'outer counter }
        }
        println!("Random print to show continue with {counter}");
    };
    println!("loop_result = {loop_result}");

    // Iterative constructs with while
    let arr: [u8; 5] = [1,2,3,4,5];
    let mut counter2: usize = arr.len();
    while counter2 != 0 {
        counter2 -= 1;
        println!("Looping with while: {}", arr[counter2]);
    }

    // Iterative constructs with for
    let arr2: [u8; 5] = [2,3,4,5,6];
    for element in arr2 {
        println!("Looping with for: {element}");
    }
}

pub fn convert_temp(input_temp: i16, temp_target: String) -> i16 {
    match temp_target.as_str() {
        "F" => (9/5)*input_temp + 32,
        "C" => (input_temp - 32)*(5/9),
        _ => panic!("Invalid temp type")
    }
}

pub fn fibonacci(n: u8) {
    let mut counter: u8 = 0;
    let mut prev1: u16 = 0;
    let mut prev2: u16 = 0;
    let mut sum: u16 = 0;
    while counter != n {
        if counter == 0 {
            counter += 1;
            println!("Fibonacci {} = {}", counter, sum);
            continue;
        } else if counter == 1 {
            sum += 1;
            counter += 1;
            println!("Fibonacci {} = {}", counter, sum);
            continue;
        }
        prev1 = prev2;
        prev2 = sum;
        sum = prev1 + prev2;
        counter += 1;
        println!("Fibonacci {} = {}", counter, sum);
    }
}