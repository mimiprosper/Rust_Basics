fn greet(word: String) {
    println!("Hello, {}!", word);
}

fn salute(word: &String) { // borrows
    println!("Hello, {}!", word);
}


/// This function demonstrates the difference between moving and borrowing values.
/// - It first calls `greet` with a `String` argument, which moves the value.
/// - It then calls `salute` with a reference to a `String` argument, which borrows the value.
/// - Finally, it attempts to print the value of the `String` argument.
///   - When the value was moved, this causes a compile error.
///   - When the value was borrowed, this works as expected.
    
fn main() {
    let name = String::from("Bitcoin");
    greet(name); // ownership moves here
    // println!("{}", name); // ❌ This line would cause an error! (use after move)

    let name = String::from("Bitcoin");
    salute(&name); // borrow instead of move
    println!("I still own: {}", name); // ✅ this works

}

