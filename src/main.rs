// each file imported as module
mod printers;
mod types;

/// Library docs for next item: The main function starts everything in Rust
fn main() {
    // functions from the file modules used inside main
    printers::demo();
    types::demo();
}
