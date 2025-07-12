# Common Collections

- Unlike built-in arrays and tuples, the data in these collections are stored on the heap
  - this allows them to grow and shrink as required since size is not required at compile time

## Vectors

- A vector allows you to store variable number of values of same type as a list
- Since we cannot have a mutable and immutable reference at the same time
  - we cannot add to a vector while also having a reference to an element of the vector
  - this is done because the entire vector maybe copied to a new location if there isn't enough memory for the new element where it is currently stored
  - holding a reference to the old location therefore is a memory issue
  - for the same reason, its not possible to insert items into vector while looping through it
- Vector can store multiple types using enums
  - this only works if all the types to be stored are known at compile time
- When a vector goes out of scope, it drops all its elements

---

## Strings

- A string is a collection of characters
- Rust strings are UTF-8 encoded
  - as a result, the number of visible characters and the number of bytes used are not always the same
  - this is also the reason why Strings don't support indexing in Rust
  - even if it were possible to index, it wouldn't be a `O(1)` operation
  - thus, we use slicing using the `..` range operator on the reference of the `String`
  - that being said, trying to slice individual characters like 0..1 can still panic due to the encoding
  - we can iterate over each character using the `.chars()` method
  - we can also iterate over the individual bytes but each byte may not correspond to each character
- The `to_string()` method is available to all types that implement the `Display` trait
- When appending strings using `+`, we can only add `&str` to `String`
  - this will always return a new result and the original `String` would no longer be available as its moved

---

## HashMap

- Allows storing a collection of keys and corresponding values
- All keys should be of same type and all values should be of same type
  - it is technically possible to have an enum type for the keys and values though
- Using variables as args in `insert` moves their value away unless they implement the Copy trait
- By default, the hashing function used is `SipHash`
  - it is slower but more secure
  - we can switch to faster hash algorithms by changing the `hasher`

---
