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

---