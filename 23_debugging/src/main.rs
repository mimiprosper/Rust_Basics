
#[allow(unused_variables)]
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::error::Error;
use std::io::{self};

// recoverable errors
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn main() {
//     // using match statement
//     let greetings_from_file = File::open("greetings.txt");

//     let greetings_file = match greetings_from_file {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("greetings.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error)
//             }
//         }
//     };
//     println!("{:?}", greetings_file);
// }

// fn main() {
//     // using if-else statement
//     let greetings_from_file = File::open("greetings.txt");

//     let greetings_file = if let Ok(file) = greetings_from_file {
//         file
//     } else {
//         File::create("greetings.txt").unwrap_or_else(|error| {
//             panic!("Problem creating the file: {}", error);
//         })
//     };
//     println!("{:?}", greetings_file);
// }

// // unwrap and expect
// fn main() {
//     //unwrap is short cut returns value if it is OK or panics if Err

//     // let greetings_from_file = File::open("greetings.txt").unwrap(); 
//     let greetings_from_file = File::open("greetings.txt").expect("Could not open file");

// }

// propagate errors

// fn read_username_from_file() -> Result<String, io::Error>{
    // let f = File::open("great.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(error) => Err(error)
    // }

    // let mut username_from_file = File::open("hello.txt");
    // let mut username = String::new();

    // username_from_file.read_to_string(&mut username)?;
    // Ok(username);

    // let fs = read_to_string("hello.txt");

// }


// using the ? operator to return the error to the calling function
fn main() -> Result<(), Box<dyn Error>> {
    let mut greeting_file = File::open("greet.txt")?;

    // print value of the file
    let mut greetings = String::new();
    greeting_file.read_to_string(&mut greetings)?;
    println!("Greeting {}", greetings);
    Ok(())
}

