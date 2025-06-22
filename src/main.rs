// each file imported as module
mod guessing_game;
mod printers;
mod types;
mod control_flow;
mod ownership;
mod structs;
mod enums;

/// Library docs for next item: The main function starts everything in Rust
fn main() {
    // functions from the guessing_game module (0)
    // guessing_game::do_guess();

    // functions from the print module (1)
    // printers::basic_demo();
    // printers::complex_demo();

    // functions from the type module (2)
    // types::basic_type_demo();
    // types::custom_structs_demo();
    // types::custom_enum_demo();
    // types::linked_list_demo();

    // functions from the control_flow module (3)
    // control_flow::basic_demo();
    // println!("0C = {}F", control_flow::convert_temp(0, String::from("F")));
    // println!("32F = {}C", control_flow::convert_temp(32, String::from("C")));
    // control_flow::fibonacci(10);

    // functions from the ownership module (4)
    // ownership::ownership_demo();
    // ownership::slices_demo();

    // functions from the structs module (5)
    // structs::structs_demo();

    // functions from the enums module (6)
    enums::enums_demo();
}
