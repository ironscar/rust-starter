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
    - immutably borrowing a single entry of a mutably borrowed HashMap is possible
- We also cannot return the reference from a function if the original value was created in the function
    - this is because the original value ceases to exist once the function ends and so it is a null reference
    - this is often called dangling pointer and Rust doesn't allow it
    - basically, if the function owns a value, return it by value instead of reference
