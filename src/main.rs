// each file imported as module
mod printers;
mod types;

/// Library docs for next item: The main function starts everything in Rust
fn main() {
    // functions from the print module
    // printers::basic_demo();
    // printers::complex_demo();

    // functions from the type module
    // types::basic_type_demo();
    // types::custom_structs_demo();
    // types::custom_enum_demo();
    types::linked_list_demo();

    // continue from https://doc.rust-lang.org/rust-by-example/variable_bindings.html
}
