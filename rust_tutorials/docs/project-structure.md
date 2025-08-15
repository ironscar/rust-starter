## Workspaces / Packages / Crates / Modules

### Workspaces

- Workspaces in Cargo are a collection of crates in a single project
- There can be any number of binary or library crates in a workspace
- Workspaces require a `Cargo.toml` at root with the workspaces tag and the corresponding members by package name
  - we can also specify resolver as `2` which is the new way of resolving packages, else it shows warnings on build 
- Each directory in this workspace can be these individual packages, each with its own `Cargo.toml` with package name
- Creating workspaces sometimes makes the file name in explorer view look red (error)
  - recreating the files with the same content seems to fix it
- To run a specific crate, run `cargo run --bin <package_name>`
- We can also set `default-run = "<package_name>"` in Cargo.toml of specific package
  - this should be specified only for one
  - then running `cargo run` runs that specific package

### Packages

- Packages are usually a bundle of crates to a provide a functionality
- Packages allow to build, test and share crates
- Packages can contain multiple binary crates and optionally one library crate
  - when this is the case, its usually that the library has most of the code to be reused
  - the binary crate merely serves as a quick executable for the library
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
- There are two ways to manage this
  - The old way
    - every internal folder can be a module if we define a `mod.rs` file in it
    - this can then declare each internal file as a submodule with `pub mod <filename>`
  - The new way
    - have the module file parallel to `main.rs` and declare the submodules there
    - the submodule files will still be inside a directory with the module name
- Finally, the module itself can be declared into `main.rs` as `mod <foldername>` and used either directly or with `use`
- We can use a submodule as `module::submodule::function()`
- Code within modules are private by default unless prefixed by `pub`
- We can also import a submodule anywhere within the crate as `use crate::module::submodule` and then directly use submodule within the file
  - the first part is `crate` because it is the implicit root within a project
  - but when using external crates, we need to use the name of that crate
  - paths can also be relative but it's generally recommended to have absolute paths
  - we can also refer to parent module members using `super::` at the start
- Submodules by default can see what their parent modules define but not vice versa
  - Both the submodule declaration and the member inside need to marked as `pub` to be visible to parent module
- A struct marked with `pub` still has private fields
  - we can mark each field with `pub` to decide encapsulation
  - same with functions inside the `impl` scope of the struct
  - all variants of the enum are automatically public if the enum is but the functions inside `impl` scope can be encapsulated
- When importing modules, we can do `use as` keywords to give the import a new name in current scope
  - we can also mark this with `pub` to make the new name accessible outside the module as by default it is private
- If we are importing multiple things from a single crate
  - such as `use std::cmp::Ordering;` and `use std::io;`
  - we can define this as `use std::{cmp::Ordering, io};` to simplify
  - we can also use `std::*` to import every public item within the module to scope