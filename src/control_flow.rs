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
}