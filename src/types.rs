/// https://doc.rust-lang.org/rust-by-example/primitives.html
/// public function to use in other modules
pub fn basic_type_demo() {
    // signed integers: i8,i16,i32 (default),i64,i128
    let a: i32 = 5;
    println!("i32 = {}", a);

    // unsigned integers: u8,u16,u32,u64,u128
    let b: u32 = 7;
    println!("u32 = {}", b);

    // floating point: f32,f64 (default)
    let c: f32 = 6.5;
    println!("f32 = {}", c);

    // char (4 bytes each) is a single character
    let d: char = 'x';
    println!("char = {}", d);

    // bool
    let e: bool = false;
    println!("bool = {}", e);

    // all variables are immutable by default and values cannot be reassigned
    // We need to create a mutable variable to change its value but not its type
    let mut mutable_var: i32 = 4;
    println!("mutable i32 before overwrite = {}", mutable_var);
    mutable_var = 6;
    println!("mutable i32 after overwrite = {}", mutable_var);

    // variables can be overwritten by shadowing as below
    let d: i32 = 15;
    println!("shadowed i32 from char = {}", d);

    // arrays (type = [Type; Size]) and used as array[index]
    let my_arr: [i32; 6] = [1,2,3,4,5,6];
    println!("array element print = {}", my_arr[0]);

    // tuple can hold values of different types and used as tuple.index
    let my_tuple: (u32, f32, char, bool, [i32;2]) = (5, 4.0, 'a', false, [1,2]);
    println!("tuple print = {}", my_tuple.2);

    // immutable string (the & is something to do with pointers and we will see later)
    let my_str: &str = "hello world";
    println!("str = {}", my_str);

    // for easier reading of numbers, we can use underscores
    println!("long num = {}", 50_000_000u32);

    // debug print is not always clean but can allow printing some complex types
    println!("array debug print = {:?}", my_arr);
    // tuples only below 12 elements can be printed this way
    println!("tuple debug print = {:?}", my_tuple);

    // all elements of an array can be initialized to same value as [value;length]
    println!("all vals same = {:?}", [0;5]);

    // slices contain the pointer to data and length of slice and has the signature &[T] where T is type of array
    // the [a..b] notation specifies including a and excluding b
    let slice: &[i32] = &my_arr[2..4];
    println!("slice = {:?}", slice);
    println!("slice length = {}", slice.len());

    // array.get(index) returns an `Option` that can be dealt with `match Some None`
    for i in 0..10 {
        match my_arr.get(i) {
            Some(x) => print!("item = {} at index = {},", x, i),
            None => print!("no items at index = {},", i)
        }
    }
}

/// https://doc.rust-lang.org/rust-by-example/custom_types.html

/*
 * ---------------------------------------------------------------------------------
 * Custom types can be made with either `enum` or `struct`
 * Constants can be created using `const` and `static`
 */

// Constants (these can be global as well)
// Specifying underscore before variable hides compiler warnings if variable is unused
const _CONSTANT_VAL: i32 = 5;
// CONSTANT_VAL = 12; // throws error
static _STATIC_CONSTANT_VAL: i32 = 10;
// STATIC_CONSTANT_VAL = 12; // throws error

// classic C structs (we need to use String if struct should own the value itself instead of &str)
#[derive(Debug)]
struct PersonStruct {
    name: String,
    age: u8
}

// A tuple struct
#[derive(Debug)]
struct TupleStruct (String, u8);

// A unit struct (useful for generics)
#[derive(Debug)]
struct Unit;

// Nested structs
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Rectangle {
    top_left_corner: Point,
    bottom_right_corner: Point
}

pub fn custom_structs_demo() {
    // PersonStruct
    let person = PersonStruct { name: String::from("Edward"), age: 8 };
    println!("personStruct name = {}, age = {}", person.name, person.age);
    println!("personStruct = {:?}", person);

    // TupleStruct
    let tuple_struct = TupleStruct(String::from("Edward"), 32u8);
    println!("tupleStruct name = {}, age = {}", tuple_struct.0, tuple_struct.1);
    println!("tupleStruct = {:?}", tuple_struct);

    // UnitStruct
    let _unit = Unit;
    println!("unit = {:?}", _unit);

    // Nested structures usage
    let rect = Rectangle {top_left_corner: Point {x: 0, y: 20}, bottom_right_corner: Point {x: 10, y: 0}};
    let width: u32 = (rect.bottom_right_corner.x - rect.top_left_corner.x) as u32;
    let height: u32 = (rect.top_left_corner.y - rect.bottom_right_corner.y) as u32;
    let area: u32 = width * height;
    println!("area is {}", area);
}

/*
 * ---------------------------------------------------------------------------------
 * This section covers enums in more detail
 */

// Enum with implicit discriminator (PageLoad = 0, KeyPress = 1, ...)
enum WebEvent {
    PageLoad, // no property
    KeyPress(char), // single property
    Paste(String), // single property
    Click(i64,i64), // multiple properties
    NamedClick { x: i64,y: i64 } // named destructuring
}

// Enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
}

// Type for Enum
type Wev = WebEvent;

fn print_event_sign(event: WebEvent) {
    match event {
        Wev::PageLoad => println!("page load"),
        Wev::KeyPress(c) => println!("pressed '{}'", c),
        Wev::Paste(s) => println!("pasted '{}'", s),
        Wev::Click(x,y) => println!("clicked ({}, {})", x, y),
        Wev::NamedClick { x,y} => println!("clicked ({}, {})", x, y)
    }
}

pub fn custom_enum_demo() {
    // To use the enum types directly (we can use * here to specify all)
    use crate::types::WebEvent::{Click, KeyPress, NamedClick, PageLoad, Paste};

    // Enums & types
    print_event_sign(PageLoad);
    print_event_sign(KeyPress('s'));
    print_event_sign(Click(20,20));
    print_event_sign(NamedClick { x: 20, y: 20 });
    print_event_sign(Paste(String::from("Apple")));

    // Explicit enum casting (below doesn't seem to work with WebEvent enum)
    println!("Explicit enum: {}", Color::Green as i32);
}

/*
 * ----------------------------------------------------------------------------------
 * Linked List implementation by enum
 */
enum List {
    Node {val: i32, next: Box<List>}, // Box is a smart pointer
    Tail
}

// We can attach methods to enum (and structs) using `impl` as below
impl List {
    fn new() -> List {
        List::Tail
    }
    fn prepend(self, value: i32) -> List {
        List::Node {val: value, next: Box::new(self) }
    }
    fn len(&self) -> u32 {
        // self has Type &List and *self has type List
        match *self {
            List::Node {val: _, ref next} => 1 + next.len(), // other node case
            List::Tail => 0 // base tail case
        }
    }
    fn stringify(&self) -> String {
        match *self {
            List::Node {val, ref next} => format!("{} -> {}", val, next.stringify()),
            List::Tail => String::from("END")
        }
    }

}

pub fn linked_list_demo() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("linked list is {}", list.stringify());
}
