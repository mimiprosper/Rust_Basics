// structs is the collecton of related data that can be 
//used to represent something in your program
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


/// The main function demonstrates the creation and manipulation of `User` struct instances:
/// - Initializes a mutable `user1` instance with specific fields and prints its details.
/// - Modifies the `name` field of `user1` and prints the updated details.
/// - Uses the `build_user` function to create another `User` instance, `user2`,
///   and prints its details.
fn main() {
    // create instance of struct
    let mut user1 = User {
        name: String::from("john"),
        email: String::from("H5lNt@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "User1: {}    Email: {}    Active: {}    Sign in count: {}",
        user1.name, user1.email, user1.active, user1.sign_in_count
    );

    user1.name = String::from("steve doe");

    println!(
        "User1: {}    Email: {}    Active: {}    Sign in count: {}",
        user1.name, user1.email, user1.active, user1.sign_in_count
    );

    // create instance of struct
    let user2 = build_user(String::from("paul@example.com"), String::from("paulson"));
    println!(
        "User2: {}    Email: {}    Active: {}    Sign in count: {}",
        user2.name, user2.email, user2.active, user2.sign_in_count
    );
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}
