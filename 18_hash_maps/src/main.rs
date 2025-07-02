// import hash map
#![allow(unused)]
use std::collections::HashMap;

/// Demonstrates the creation and manipulation of HashMaps in Rust:
/// - Creates a HashMap to store team scores and inserts key-value pairs.
/// - Retrieves values from the HashMap using keys, with a default value if the key is not found.
/// - Iterates over a HashMap storing programming languages and their rankings.
/// - Demonstrates HashMap ownership by inserting key-value pairs.
/// - Uses the `entry` API to insert a value only if the key doesn't exist.

fn main() {
    let mut scores = HashMap::new();
    // Add values to the hash map using key-value pair
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // get value from hash map using the key
    let team_name = String::from("Blue"); // key
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}", score);

    let no_name = String::from("Green"); // key not found, value returns 0
    let score = scores.get(&no_name).copied().unwrap_or(0);
    println!("{:?}", score);

    // iterate over hash map
    let mut programming_languages = HashMap::new();
    programming_languages.insert(String::from("Rust"), 1);
    programming_languages.insert(String::from("C++"), 2);
    programming_languages.insert(String::from("R"), 3);
    for (key, value) in &programming_languages {
        // println!("{}: {}", key, value);
    }

    // hashmaps and ownership
    let mut choice = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    choice.insert(field_name, field_value);
    println!("{:?}", choice);

    // only inserting the value if the key has no value
    let mut map_five  = HashMap::new();
    map_five.insert(String::from("Blue"), 10);

    map_five.entry(String::from("Yellow")).or_insert(2);
    map_five.entry(String::from("Blue")).or_insert(3);

    println!("{:?}", map_five);
}
