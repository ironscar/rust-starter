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