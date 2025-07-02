// This shows how to import a module 
use crate::garden::vegetables::Asparagus;
mod garden;

/// Prints a sample Asparagus plant to the console.
fn main() {
    let plant = Asparagus {
        name: String::from("Green"),
        stalks: 10,
    };

    println!("Plant: {:?}", plant);
}

