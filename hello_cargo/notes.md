# Cargo

- Rust's build system and package manager
- Helps build your code + download and build dependencies

## cargo.toml

- Tom’s Obvious, Minimal Language
- [package]: configuration information needed to compile program such as name, version, Rust edition
- [dependencies]: list any of your project’s dependencies aka "crates"

## Commands

- `cargo new` to create a project
- `cargo build` to build program (creates an executable file in "target/debug/hello_cargo")
- Run the executable with `./target/debug/hello_cargo`
- Running `cargo build` for the first time also causes Cargo to create a new file at the top level: "Cargo.lock". This file keeps track of the exact versions of dependencies in your project
- `cargo run` to compile the code and then run the resulting executable all in one command
- `cargo check` quickly checks your code to make sure it compiles but doesn’t produce an executable
- `cargo build --release` to compile it with optimizations. This command will create an executable in 'target/release' instead of 'target/debug'. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile
