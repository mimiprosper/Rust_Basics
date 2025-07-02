use std::fs::File;
use std::io::Error;

// 🚨 1. Recoverable Errors: Result<T, E>
// Use the Result type when the error can be handled gracefully.

// 🧠 Explanation:
// File::open returns Result<File, std::io::Error>.
// The ? operator propagates the error if one occurs.
// If successful, you get the file handle.

fn error_one() -> Result<(), Error> {
    let file = File::open("myfile.txt")?;
    println!("File opened successfully!");
    Ok(())
}

// ❌ 2. Unrecoverable Errors: panic!
// Use when something should never happen, like logic bugs or unrecoverable states.

fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("Can't divide by zero!");
    }
    x / y
}


// 🛠 Custom Error Types
// You can define your own errors using enum:
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    InvalidInput,
}


/// The main function demonstrates error handling in Rust:
/// - It attempts to open a file and uses pattern matching to handle
///   the `Result` type, printing success or error messages accordingly.
/// - It calls the `divide` function with a divisor of zero, 
///   which results in a panic due to division by zero.

fn main() {
    let result = File::open("myfile.txt");

    // 📦 Manual Pattern Matching:
    match result {
        Ok(file) => println!("Opened file: {:?}", file),
        Err(e) => println!("Error opening file: {}", e),
    }

    let result = divide(10, 0); // This will panic
}