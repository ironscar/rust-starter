/// https://doc.rust-lang.org/rust-by-example/hello.html
/// public function to use in other modules
pub fn demo() {
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
