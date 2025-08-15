mod trials;

fn main() {
    let current_trial_index = 1;
    match current_trial_index {
        1 => {
            // finally managed to do a linked list implementation somewhat on my own
            trials::linked_list::linked_list_trial();
        },
        2 => {
            // some basic programming problems in Rust
            trials::basic_problems::basic_problem_1();
            trials::basic_problems::basic_problem_2();
            trials::basic_problems::basic_problem_3();
            trials::basic_problems::basic_problem_4();
            trials::basic_problems::basic_problem_5();
            trials::basic_problems::basic_problem_6();
        },
        3 => {
            // some medium programming problems in Rust
            trials::medium_problems::medium_problem_1();
        },
        _ => {
            println!("Unknown trial");
        }
    };
}