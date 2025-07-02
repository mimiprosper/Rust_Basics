// DATA TYPES
// scalars (intergers, floating-points numbers, booleans, characters)
// compound data types (tuples, arrays)
// custom data types (structs, enums)
//https://doc.rust-lang.org/book/ch03-02-data-types.html

/// The main function demonstrates basic data types in Rust:
/// - Iterates over an array and prints each element.
/// - Destructures a tuple and prints each of its values.
/// - Defines and uses a `User` struct, printing its fields.
/// - Matches against an enum `Color` and prints the corresponding color.

#[allow(unused)]
fn main() {
    let arr = [1, 2, 3, 4, 5];

    //print all elements in the array
    for i in arr.iter() {
        println!("{}", i);
    }

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // structs
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    let user1 = User {
        email: String::from("a@b.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };

    println!("{} is signed in: {}", user1.username, user1.active);
    println!("{} has signed in {} times", user1.username, user1.sign_in_count);
    println!("{}'s email is: {}", user1.username, user1.email);

    // emums
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;

    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

}

