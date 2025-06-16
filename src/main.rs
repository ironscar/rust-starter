// each file imported as module
mod printers;
mod types;
mod ownership;
mod guessing_game;
mod control_flow;

/// Library docs for next item: The main function starts everything in Rust
fn main() {
    // functions from the print module
    // printers::basic_demo();
    // printers::complex_demo();

    // functions from the guessing game tutorial
    // guessing_game::do_guess();

    // functions from the type module
    // types::basic_type_demo();
    // types::custom_structs_demo();
    // types::custom_enum_demo();
    // types::linked_list_demo();

    // control flow
    // control_flow::basic_demo();
    // println!("0C = {}F", control_flow::convert_temp(0, String::from("F")));
    // println!("32F = {}C", control_flow::convert_temp(32, String::from("C")));
    control_flow::fibonacci(10);

    // functions from the ownership module
    // ownership::ownership_demo();
}
