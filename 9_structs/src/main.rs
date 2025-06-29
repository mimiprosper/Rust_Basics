// structs example
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

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
