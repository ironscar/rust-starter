// need this to get standard input
use std::io;

// need the Ordering enum to compare anything
use std::cmp::Ordering;

pub fn do_guess() {
    // other variables including for loop control
    let actual_num: u8 = 4;
    let mut num_tries: u8 = 0;
    let max_tries: u8 = 3;

    // loop at most 5 times and stop once guessed correctly
    loop {
        println!("Please input your guessed number: ");

        // this creates a new mutable String object
        // initialized inside loop as otherwise it fails parsing for next iterations even if number
        let mut guess = String::new();

        // this is how we take the input and store it in a string variable
        // read_line returns Result so we can use expect to use value if there, else fail with msg
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // trim can remove leading and trailing spaces as well as \r\n characters
        // parse can parse anything that implements the FromStr trait
        // returns a Result so we can also match on the Ok and Err values
        // return the value from Ok and use shadowing to store it back into same variable
        // if error, then we go to the next loop iteration after checking for max_tries
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                num_tries += 1;
                if num_tries == max_tries {
                    break;
                }
                println!("Please type a number");
                continue;
            }
        };

        // check the numbers using match where the argument always takes a reference
        // break if correct guess to come out of loop
        match guess.cmp(&actual_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly, you win!");
                break;
            }
        }

        // increment num_tries and break if 5 and use wildcard match to do nothing for Lesser
        // use panic for greater as that should never happen which immediately terminates the thread
        // we use match here instead of if for the sake of showing a different way
        num_tries += 1;
        match num_tries.cmp(&max_tries) {
            Ordering::Equal => {
                println!("No more tries left, you lose!");
                break;
            },
            Ordering::Greater => panic!(),
            _ => {}
        }
    }
}
