// tutorials & trials private modules defined
mod tutorials;
mod trials;

// use guessing game submodule with absolute path
use crate::tutorials::guessing_game;

// use printers submodule with relative path
use tutorials::printers;

/// Library docs for next item: The main function starts everything in Rust
fn main() {
    // run config
    let current_tutorial = 8;
    let current_trial = 2;
    // let run_type = "tutorials";
    let run_type = "trials";

    // run actual
    if run_type == "tutorials" {
        tutorials(current_tutorial);
    } else {
        trials(current_trial);
    }
}

fn tutorials(current_tutorial: i32) {
    match current_tutorial {
        0 => {
            // functions from the guessing_game module (0)
            guessing_game::do_guess();
        },
        1 => {
            // functions from the print module (1)
            printers::basic_demo();
            printers::complex_demo();
        },
        2 => {
            // functions from the type module (2)
            tutorials::types::basic_type_demo();
            tutorials::types::custom_structs_demo();
            tutorials::types::custom_enum_demo();
        },
        3 => {
            // functions from the control_flow module (3)
            tutorials::control_flow::basic_demo();
            println!("0C = {}F", tutorials::control_flow::convert_temp(0, String::from("F")));
            println!("32F = {}C", tutorials::control_flow::convert_temp(32, String::from("C")));
            tutorials::control_flow::fibonacci(10);
        },
        4 => {
            // functions from the ownership module (4)
            tutorials::ownership::ownership_demo();
            tutorials::ownership::slices_demo();
        },
        5 => {
            // functions from the structs module (5)
            tutorials::structs::structs_demo();
        },
        6 => {
            // functions from the enums module (6)
            tutorials::enums::enums_demo();
        },
        7 => {
            // call a submodule function (7)
            trials::linked_list::linked_list_trial();
        },
        8 => {
            // common collections (8)
            tutorials::collections::vectors_demo();
        }
        _ => {
            println!("Unknown tutorial");
        }
    };
}

fn trials(current_trial: i32) {
    match current_trial {
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
        },
        _ => {
            println!("Unknown trial");
        }
    };
}
