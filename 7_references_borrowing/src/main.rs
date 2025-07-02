// ✅ Example 1: Immutable Borrowing (Read Only)
/// This function demonstrates immutable borrowing:
/// - It creates a `String` instance with the value "Bitcoin".
/// - It passes an immutable reference to `greet`, which just reads the value.
/// - It prints the value of the `String` again, showing that it's still usable.
fn greet(word: &String) {
    println!("Hello, {}!", word); // just read, no ownership taken
}

// ✅ Example 2: Mutable Borrowing (Read and Write)
// This function demonstrates mutable borrowing:
// - It creates a `String` instance with the value "Hello".
// - It passes a mutable reference to `add_world`, which modifies the value.
// - It prints the modified value, showing that it's still usable.
fn add_world(text: &mut String) {
    text.push_str(", world!"); // modify borrowed value
}


/// This is the main function
///
/// It demonstrates:
/// - Immutable borrowing: `greet` takes an immutable reference to a `String`.
/// - Mutable borrowing: `add_world` takes a mutable reference to a `String`.
/// - The borrow checker ensures thread safety and prevents data races.

fn main() {
    let name = String::from("Bitcoin");
    greet(&name); // borrow the value immutably
    println!("Back in main: {}", name); // still usable here!

    let mut message = String::from("Hello");
    add_world(&mut message); // mutable borrow
    println!("Final message: {}", message);
}


