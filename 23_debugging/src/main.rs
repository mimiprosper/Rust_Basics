
#[allow(unused_variables)]
use std::fs::File;
use std::io::{ Read};
use std::error::Error;

// This Rust code attempts to:

// 1. Open a file named "greet.txt" in the current directory.
// 2. Read the contents of the file into a string.
// 3. Print the contents of the file to the console.

// The `?` operator is used to propagate any errors that occur during 
// file opening or reading to the caller, allowing the program to exit early if an error occurs. 
// The `Result` return type of the `main` function indicates that it may return an error.
fn main() -> Result<(), Box<dyn Error>> {
    let mut greeting_file = File::open("greet.txt")?;

    // print value of the file
    let mut greetings = String::new();
    greeting_file.read_to_string(&mut greetings)?;
    println!("Greeting {}", greetings);
    Ok(())
}

