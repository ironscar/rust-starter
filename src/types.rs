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
 * Custom types can be made with either `enum` or `struct`
 * Constants can be created using `const` and `static`
 */

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

pub fn custom_type_demo() {
    // PersonStruct
    let name: String = String::from("Edward");
    let age: u8 = 32;
    let person = PersonStruct { name, age };
    println!("personStruct name = {}, age = {}", person.name, person.age);
    println!("personStruct = {:?}", person);

    // TupleStruct
    let tuple_struct = TupleStruct(String::from("Edward"), 32u8);
    println!("tupleStruct name = {}, age = {}", tuple_struct.0, tuple_struct.1);
    println!("tupleStruct = {:?}", tuple_struct);

    // UnitStruct
    let _unit = Unit;
    println!("unit = {:?}", _unit);

    // Enums

    // Constants (these can be global as well)
    const CONSTANT_VAL: i32 = 5;
    // CONSTANT_VAL = 12; // throws error
    static STATIC_CONSTANT_VAL: i32 = 10;
    // STATIC_CONSTANT_VAL = 12; // throws error

    // Nested structures
}
