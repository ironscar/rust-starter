// tutorials & trials private modules defined
mod tutorials;
mod trials;
mod automl_factory;

// use guessing game submodule with absolute path
use crate::tutorials::guessing_game;

// use printers submodule with relative path
use tutorials::printers;

/// Library docs for next item: The main function starts everything in Rust
fn main() {
    // run config
    let current_tutorial = 9;
    let current_trial = 1;
    let current_automl_impl = 1;
    // let run_type = "tutorials";
    // let run_type = "trials";
    let run_type = "automl";

    // run actual
    if run_type == "tutorials" {
        tutorials(current_tutorial);
    } else if run_type == "trials" {
        trials(current_trial);
    } else if run_type == "automl" {
        automl_factory_impls(current_automl_impl);
    }
}

fn tutorials(current_tutorial: i32) {
    match current_tutorial {
        1 => {
            // functions from the guessing_game module (1)
            guessing_game::do_guess();
        },
        2 => {
            // functions from the print module (2)
            printers::basic_demo();
            printers::complex_demo();
        },
        3 => {
            // functions from the type module (3)
            tutorials::types::basic_type_demo();
            tutorials::types::custom_structs_demo();
            tutorials::types::custom_enum_demo();
        },
        4 => {
            // functions from the control_flow module (4)
            tutorials::control_flow::basic_demo();
            println!("0C = {}F", tutorials::control_flow::convert_temp(0, String::from("F")));
            println!("32F = {}C", tutorials::control_flow::convert_temp(32, String::from("C")));
            tutorials::control_flow::fibonacci(10);
        },
        5 => {
            // functions from the ownership module (5)
            tutorials::ownership::ownership_demo();
            tutorials::ownership::slices_demo();
        },
        6 => {
            // functions from the structs module (6)
            tutorials::structs::structs_demo();
        },
        7 => {
            // functions from the enums module (7)
            tutorials::enums::enums_demo();
        },
        8 => {
            // common collections (8)
            tutorials::collections::vectors_demo();
            tutorials::collections::strings_demo();
            tutorials::collections::hashmap_demo();
            tutorials::collections::collections_exercises_1();
            tutorials::collections::collections_exercises_2();
            tutorials::collections::collections_exercises_3();
        },
        9 => {
            // error handling (9)
            // tutorials::error_handling::default_panic_demo();
            // tutorials::error_handling::recoverable_errors_demo_1();
            // tutorials::error_handling::recoverable_errors_demo_2();
            tutorials::error_handling::error_propagation_demo();
        },
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

fn automl_factory_impls(current_automl_impl: i32) {
    match current_automl_impl {
        1 => {
            automl_factory::linear_regression::linear_regression_demo();
        },
        _ => {
            println!("Unknown automl implementation");
        }
    };
}
