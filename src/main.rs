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
    types::custom_type_demo();
}
