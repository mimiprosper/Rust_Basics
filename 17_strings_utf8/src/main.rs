
/// Demonstrates various string operations in Rust:
/// - Creates and appends to a mutable `String`.
/// - Converts a string literal to a `String` and prints it.
/// - Handles UTF-8 encoded strings.
/// - Concatenates strings using `+` operator and `format!`.
/// - Demonstrates string slicing and iterating over characters and bytes.
/// - Concatenates multiple strings using both `+` operator and `format!`.
fn main() {
    // create a string
    let mut s1 = String::from("hello");

    // append to the string
    s1.push_str(", world!");

    println!("{} 🗺️", s1);

    // create another string
    let s2 = "blue palnet".to_string();
    println!("{} 👌", s2);

    // utf-8 encoded string
    let hello = String::from("السلام عليكم 👍");
    println!("{}", hello);

    // concatenation
    let s3 = "Great ".to_string();
    let s4 = "Team".to_string();

    // let s3 = String::from("Great ");
    // let s4 = String::from("Team");
    let s5 = s3 + &s4;
    println!("{}", s5);

    let one = String::from("one");
    let two = String::from("two");
    let three = String::from("three");

    let format_them = format!("{}-{}-{}", one, two, three);
    println!("{}", format_them);
    // println!("{}-{}-{}", one, two, three);

    // concatenate 4 strings
    let s6 = String::from("I like ");
    let s7 = String::from("ice cream ");
    let s8 = String::from("and ");
    let s9 = String::from("chocolate");
    let s10 = s6 + &s7 + &s8 + &s9;
    println!("{}", s10);

    // There is no indexing in strings, but there is slicing strings
    let hello_in_russian = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // iterate over a string
    for russian in hello_in_russian.chars() {
        println!("{}", russian);
    }
    
    // iterate over bytes
    for byte in hello_in_russian.bytes() {
        println!("{}", byte);
    }
}
