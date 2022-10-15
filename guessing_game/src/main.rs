// import input/output library 
// the io library comes from the standard library, known as std:
use std::io;

// By default, Rust has a set of items defined in the standard library 
// that it brings into the scope of every program. 
// This set is called the prelude, and you can see everything in it 
// in the standard library documentation.

// Crate = a collection of Rust source code files. 
// This project is a binary crate, which is an executable. 
// The `rand` crate is a library crate, which contains code intended to be used in other programs 
// and can't be executed on its own.

// Before we can write code that uses rand, 
// we need to modify the Cargo.toml file to include the rand crate as a dependency
use rand::Rng;
// Rng is a `trait` which defines methods that random number generators implement.

// The Ordering type is another enum and has the variants Less, Greater, and Equal. 
// These are the three outcomes that are possible when you compare two values.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // `rand::thread_rng` function gives us a particular random number generator:
    // one that is local to the current thread of execution and seeded by the operating system. 
    // Then we call the `gen_range` method on the random number generator. 
    // This method is defined by the Rng trait that we brought into scope with the use rand::Rng statement. 
    // The gen_range method takes a range expression as an argument and generates a random number in the range. 
    // The kind of range expression we’re using here takes the form `start..=end` 
    // and is inclusive on the lower and upper bounds, 
    // so we need to specify 1..=100 to request a number between 1 and 100.

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}!");

    println!("Please input your guess.");

    // In Rust, variables are immutable by default, 
    // meaning once we give the variable a value, the value won't change.
    // To make a variable mutable, we add `mut` before the variable name:
    let mut guess = String::new();

    // `String::new` = a function that returns a new instance of a String. 
    // String is a string type provided by the std library that is a growable, UTF-8 encoded bit of text.

    // The `::` indicates that new is an associated function of the String type. 
    // An associated function is a function that’s implemented on a type, in this case String. 
    // This new function creates a new, empty string. 
    // You’ll find a new function on many types, 
    // because it’s a common name for a function that makes a new value of some kind.

    // Call the stdin function from the io module
    // If we hadn’t imported the io library with use std::io at the beginning,
    // we could still use the function by writing this function call as `std::io::stdin`.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    //  `.read_line(&mut guess)` calls the read_line method on the std input handle 
    // to get input from the user. 
    // We’re also passing `&mut guess` as the argument to read_line to tell it 
    // what string to store the user input in. 
    // The string argument needs to be mutable so the method can change the string’s content.

    // The `&` indicates that this argument is a reference, 
    // which gives you a way to let multiple parts of your code 
    // access one piece of data without needing to copy that data into memory multiple times. 

    // Like variables, references are immutable by default. 
    // Hence, you need to write `&mut guess` rather than `&guess` to make it mutable.

    // .read_line() returns a Result value
    // `Result` is an enumeration (enum)
    // which is a type that can be in one of multiple possible states. 
    // We call each possible state a variant.
    // Result's variants are Err and Ok (these values have methods on them)

    // An instance of Result has an `expect` method that you can call. 
    // If Err, `expect` will cause the program to crash and display the message 
    // that you passed as an argument to expect. 
    // If this instance of Result is an Ok value, 
    // expect will take the return value that Ok is holding and return just that value. 
    // In this case, that value is the number of bytes in the user’s input.

    // If you don’t call expect, the program will compile, but you’ll get a warning:
    // `warning: unused `Result` that must be used` (enforced error handling)

    println!("You guessed: {guess}");

    // {} = placeholder
    // Printing multiple values in one call to println! would look like this:
    // println!("x = {} and y = {}", x, y);
}
