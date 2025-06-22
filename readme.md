# Rust Starter

## Rust

- Rust is a systems programming language focusing on safety, speed and concurrency
- It doesn't use a garbage collector

## Rust Rover IDE

- We began with downloading `Rust Rover` IDE but that doesn't seem to install Rust directly
- We had to set up manual proxy for TI laptop and do the activation while on VPN
- We can set the `Settings > Keymap` to `VSCode`
- So we will begin with setup following https://www.rust-lang.org/learn/get-started
- We can swap between files quickly with `Ctrl+Tab` and `Ctrl+Shift+Tab`

---

## Rust up

- `Rustup` is a Rust installer and version management tool
- We download the `64-bit exe` and then run it
- We need to get off VPN for the downloads it does
- Then we can restart the IDE
- We can run `rustup update` to get the latest version of rust
- Current version is `1.83.0` but you can check with `rustc --version`
- This also installs `cargo` which is Rust's package manager
- You can get its version by running `cargo --version` which is same as above for now

---

## Running programs

- We can run `cargo new <project-name>`
- Ideally we should open this in the IDE
- This generates a manifest file called `Cargo.toml` (similar to `package.json` for node)
  - includes metadata and dependencies
- It also generates a basic `main.rs` file with a `print` statement (where we will write our code)
- Running `cargo run` compiles and runs the program

---

## Adding dependencies

- Dependencies are available at `crates.io` 
- We add dependencies in the `Cargo.toml` file and RustRover gives intellisense here for package names and versions
- We can also run `cargo add <dependency>@<version>` to do the same thing
- Then `cargo build` will install the dependency
- We write the code given in example (for now let's not worry what it means) and run `cargo run`
- The program successfully uses the dependency to give output
- To know the documentation of a particular dependency, we can run `cargo doc --open`
  - this generates the docs and opens in local browser
- There is a concept of lock file called `Cargo.lock` here as well
  - this will lock the dependency versions to whatever is specified in the lock file unless updated
  - `cargo update` can be run to update the lock file to latest versions
  - this helps in dependency management to exact versions lest there are breaking changes

---

## Ownership

- Assume a complex object was created and stored in a variable `s1`
  - also assume the complex object type doesn't have the `Copy` trait
- Assume a new variable `s2` was created as `s2 = s1`
- The way it works is that `s2` gets the pointer value of `s1` and both pointers are stored in stack
- But the actual data the pointer points to is stored on the heap and isn't copied
- So during creation of `s1`, heap space is allocated for the object
- But when both `s1` and `s2` go out of scope, the space for this data is tried to be deallocated twice
- But deallocating twice will cause memory errors
- To avoid this, `s1` gets dropped the moment its value is assigned to `s2` in Rust
- Then when `s2` goes out of scope, heap space is only deallocated once as `s1` is already dropped
  - this is often called `move` and is similar to shallow-copy, but you can no longer use the old variable
  - this only comes into play for non-basic types as basic type sizes are known and are easy to copy directly on to the stack
- If we want to make deep copies, we can do so with `.clone()` which copies the data in the heap space as well
  - this can be expensive
- Rust provides a trait called `Copy` that allows you to mark other types with the ability to be deep-copied like basic types and thus never move
- When using functions, if a complex type is passed as parameter, it gets dropped after being passed into the function
  - this implies that if we want to use that param in the calling function again, we would have to return it back along with any other values
  - we can still do this by returning a tuple from the function, but it is tedious
  - then we come to the concept of `References` that lets us use values without transferring ownership
  - in general, Rust shouldn't take ownership of arguments unless required and get values by reference

## References & Borrowing

- A reference is like a pointer to the value but unlike a pointer, it can never be null
- We specify a reference value by using `&` as prefix to the variable or type, called `referencing` operator
  - the opposite is the dereferencing operator denoted by `*`
- The variable that stores the reference doesn't own the value and hence doesn't deallocate any heap space when it goes out of scope
- Creating a reference is called `borrowing`
- The value of a reference cannot be changed as its immutable and doesn't own the value
  - but we can clone the value from a reference and then edit that
  - again however, we have to return it from the function so that we can use it
- That being said, there is another way to change the value of a reference, which is `mutable references`
  - specified by a prefix of `&mut `
  - the original variable needs to be a mutable for this to work
  - we can have only one mutable reference and no other references (mutable or otherwise) are allowed
  - we can however have multiple immutable references at the same time
  - we cannot create a mutable reference if there exists an immutable reference already
- We also cannot return the reference from a function if the original value was created in the function
  - this is because the original value ceases to exist once the function ends and so it is a null reference
  - this is often called dangling pointer and Rust doesn't allow it
  - basically, if the function owns a value, return it by value instead of reference

---

## Structures

- It is a way to group related data into a single structure
- Structs are more flexible than tuples as we can name each element of a struct
- When instantiating, the fields don't have to be defined in same order as definition
  - but all of them must be defined
  - if we want the values to be mutated, then we can mark the variable mutable
- We can create tuple-like structs to treat same looking tuples differently by name
  - unlike tuple destructuring, for these we use `let struct_name (x,y) = struct_var;`
- We can also have UnitStructs that don't store anything (which become useful in defining certain kinds of traits)
- Struct properties cannot be copied with references intact
  - which means if we are using one struct to copy values to another, the first struct will be moved (cannot be used)
  - Rust makes this happen as otherwise there would be dangling references in the first struct
  - this only happens for properties in struct that don't have `Copy` trait
  - Not having copy trait attempts to make a reference and so fails to compile
  - If the properties with copy traits are only copied, the first struct would still be valid
- Structs generally own the values for their properties
  - implying it would be preferred to use String instead of &str
  - but they can also store references if required but with the use of something called `lifetimes` covered later
  - Lifetime parameters make sure that the reference values used are valid as long as the struct is
- In Rust, we can also associate methods to structs with the `impl` keyword
  - methods have the `self` object as first argument so that they can be called with dot(.) operator
  - it can take ownership of self (no reference) as well but generally better to do by reference
  - we can define this parameter as `&self` if it's a getter or `&mut self` if it's a setter
    - this is a shorthand for `self: &Self` where `Self` is an alias for the type of the `impl` block
  - Rust also has automatic referencing and dereferencing
    - In C++, we have to use `->` operator to call methods on object if using pointer
    - In Rust, it automatically figures out if caller is reference or not and adjust accordingly
    - So `p1.method() and (&p1).method()` are same so we can always write the first
  - Methods which don't have self as argument cannot be called with an instance
    - these are similar to static methods and are often used as constructors
    - Constructors inside `impl` block can choose to return type as `Self`
  - Structs can also have multiple `impl` blocks which can be useful sometimes like generics discussed later

---

## Enums

- Enums can also be used to add custom behavior in Rust
  - specifically when an instance can be of exactly one type from a list of types
  - and all types share common behavior (using the `impl` keyword)
- They can either have automatic discriminators or custom ones specified by `=`
- They can also implicitly define tuples or structs
- One important enum which is widely used is the `Option` enum
  - specifies whether `Some` value exists or is it `None`
  - Rust doesn't have `null` and `Option` becomes the alternative
  - We can use `match` with the `Some` and `None` values accordingly
- If matching a single pattern in `Option`, we can use `match `with `_` but it is verbose
  - so we can use `if let` and this can also take an else block after `}`
  - if we want to return if some condition is not met, we can use `let else`

---

Next, we will continue with https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
