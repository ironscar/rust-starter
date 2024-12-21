# Rust Starter

## Rust

- Rust is a systems programming language focusing on safety, speed and concurrency
- It doesn't use a garbage collector

## Rust Rover IDE

- We began with downloading `Rust Rover` IDE but that doesn't seem to install Rust directly
- We had to set up manual proxy for TI laptop and do the activation while on VPN
- We can set the `Settings > Keymap` to `VSCode`
- So we will begin with setup following https://www.rust-lang.org/learn/get-started

---

## Rustup

- `Rustup` is a Rust installer and version management tool
- We download the `64-bit exe` and then run it
- We need to get off VPN for the downloads it does
- Then we can restart the IDE
- We can run `rustup update` to get the latest version of rust
- Current version is `1.83.0`
- This also installs `cargo` which is Rust's package manager
- You can get its version by running `cargo --version`

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

---

Next, we will continue with https://doc.rust-lang.org/rust-by-example/
