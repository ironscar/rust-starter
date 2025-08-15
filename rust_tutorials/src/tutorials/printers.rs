use std::fmt;

/// https://doc.rust-lang.org/rust-by-example/hello.html
/// public function to use in other modules
pub fn basic_demo() {
    // this is a line-comment
    // println! is a macro (notice the ! at the end of println)
    println!("Hello, world!");

    /*
      This is a block comment
      This can be multiple lines till end of tag
    */
    println!("I am a Rustacean!");

    /*
      printing and formatting it is done by macros defined in std::fmt
      print macros writes to console (io:stdout) whereas format macros writes to String
      there is print, println for normal and new-line prints (format has its own versions for this)
      there is also eprint and eprintln counterparts for errors (io:stderr) (this doesn't have format versions)
    */

    // we can use placeholders like below
    println!("{} days", 31);

    // placeholders can also be positional
    println!("{0} to {1} days is better than {1} - {0}", 30, 40);

    // placeholders can be named
    println!("{name1} takes care of {name2}", name1 = "Jack", name2 = "Alice");
}

/*
 * -------------------------------------------------------------------------------------------------
 * std::fmt has two main traits for printing and formatting: fmt::Debug and fmt::Display
 * To be printable, it needs to derive from these traits
 * All types can derive from fmt::Debug automatically as `#[derive(Debug)]` and prints with `{:?}`
 * fmt::Display on the other hand has to manually implemented to control the display
 * Traits are used to do behavioral-inheritance as Rust doesn't have classic inheritance
 */

// unprintable structure as it neither derives fmt::Debug, not implements fmt::Display
struct Unprintable(i32);

// Debug-printable structure as it derives fmt::Debug but cannot use fmt::Display
#[derive(Debug)]
struct DebugPrintable(i32);

// Display-printable structure as it implements fmt::Display, but cannot use fmt::Debug
struct DisplayPrintable(i32, i32);
impl fmt::Display for DisplayPrintable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* notice how we don't end with a semicolon for the last write!
         * this is because each write! returns fmt::Result
         * If we want to have multiple writes, we must use `?` after `write!` followed by semicolon
         * It implies if error, return error, else continue
         * This way we can print more complicated structures as well in multiple statements like lists
         * `write!` allows us to format the display how we want
         * this is the exact syntax to follow to implement the fmt::Display trait
         */
        // write!(f, "a = {}, b = {}", self.0, self.1)
        write!(f, "a = {},", self.0)?;
        write!(f, "b = {}", self.1)
    }
}

pub fn complex_demo() {
    // println!("Unprintable = {:?}", Unprintable(3)); // throws error
    // println!("Unprintable = {}", Unprintable(3)); // throws error
    println!("Debug printable = {:?}", DebugPrintable(3));
    // println!("Debug printable = {}", DebugPrintable(3)); // throws error
    println!("Display Printable = {}", DisplayPrintable(4,5));
    // println!("Display Printable = {:?}", DisplayPrintable(4,5)); // throws error
}
