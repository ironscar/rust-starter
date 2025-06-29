## Packages / Crates / Modules

### Packages

- Packages are usually a bundle of crates to a provide a functionality
- Packages allow to build, test and share crates
- Packages can contain multiple binary crates and optionally one library crate
- Packages contain the `Cargo.toml` to specify how to build the crates in it
- To have multiple binary crates, there would be multiple files under `src/bin`

### Crates

- A tree of modules that produces a library or executable
- It is the smallest amount of code that Rust considers while compiling
- Crates can either be binary crates or library crates
- Binary crates are crates that can be executed and needs to have a `main` function
- Library crates cannot be executed and don't have a `main` function, they are only meant to be used by other crates
- The crate root is the source file that Rust begins compiling from, usually the `main.rs` for binary crates and `lib.rs` for library crates

### Modules

- Modules can be used to manage multiple files with access specifiers on internals
- Every internal folder can be a module if we define a `mod.rs` file in it
- This can then export each internal file as `pub mod <filename>`
- Finally, the module itself can be imported into `main.rs` as `mod <foldername>`
- We can use a submodule as `module::submodule::function()`
