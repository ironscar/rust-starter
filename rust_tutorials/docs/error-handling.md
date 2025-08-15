# Error-Handling

- There are no exceptions in Rust, instead there is `panic` and recoverable errors with `Result<T,E>`

## Panic

- By default, panic will print an error message and then unwind the stack to clean up data
  - This unwinding can sometimes take time and we can choose whether or not to clean up as well
  - we can do this by setting `panic = 'abort'` under `[profile.release]` in `Cargo.toml`
- Panic will usually show the specific line where the error occurs
- We need to set an env variable `RUST_BACKTRACE=1` to enable backtrace of the error
  - for more verbosity, we can set value as `full` and `0` to disable

---

## Recoverable errors

- `Result` is an enum with two values `Ok(T)` and `Err(E)` 
  - `T` is the generic type of value in case of success
  - `E` is the genetic type of error in case of failure

### Matching different errors

- This is when we want to react to different kind of errors differently
- The `Error` struct has a `kind` method implementation which returns an `ErrorKind` enum value
- We can `match` on the various enum values to react differently

### Shortcuts to panic

- We can use `unwrap()` directly to get value out of `Option` if `Ok` else it panics
- We can use `expect()` to do the above and also add a custom log before it panics

### Propagating Errors

- Its like when we throw an exception from a function to be handled outside the function
- Here in `error_thrower` we return a `Result` enum with a success string if successful and error with string value
  - Then in `error_propagation_demo` we handle that error by printing the error message
  - This is the explicit manner of propagating errors with custom messages
- If we wanted to just implicitly propagate them outside, we could use `?` after a `Result` or `Option`
  - To use this on a return type, that type needs to implement the trait `FromResidual`
  - The `?` cannot be used on a reference of the `Result` instance
  - While using with `Option`, function must return `Option` too

---
