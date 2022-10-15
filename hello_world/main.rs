// The main function is special: 
// it is always the first code that runs in every executable Rust program.

// NOTES:

// Rust style is to indent with four spaces, not a tab.

// Using a ! means that you’re calling a Rust macro 
// instead of a normal function, 
// and macros don’t always follow the same rules as functions.

// Most lines of Rust code end with a semicolon!

// Compiling and Running are separate steps (`rustc main.rs`)
// ./main.rs is the source code ./main is the executable

// Rust is an ahead-of-time compiled language, 
// meaning you can compile a program and 
// give the executable to someone else, 
// and they can run it even without having Rust installed.

fn main() {
    println!("hello world");
}