use std::collections::HashMap;

mod front_of_house {
    pub mod hosting {

/// Adds a customer to the waitlist.
///
/// This function prints a message indicating that a customer has been added
/// to the waitlist. It is typically called within the context of managing
/// restaurant seating and reservations.
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }
    }
}

mod eat_at_restaurant{
    use crate::front_of_house::hosting;

/// Serves an order by adding a customer to the waitlist.
///
/// This function calls the `add_to_waitlist` function from the `hosting` module,
/// indicating that an order is being served by adding the customer to the waitlist.
    pub fn serve_order() {
        hosting::add_to_waitlist();
    }
}


/// This is the main function.
///
/// It demonstrates:
/// - Calling functions from different modules with the `use` keyword.
/// - Creating a `HashMap` and inserting key-value pairs.
/// - Importing the `rand` crate and using a random number generator.
/// - Importing the entire `std::io` module using the glob operator.

fn main() {
    use crate::front_of_house::hosting;

    hosting::add_to_waitlist();
    eat_at_restaurant::serve_order();

    let mut map = HashMap::new();
    map.insert("key1", "value2");
    map.insert("key2", "value2");
    println!("{:?}", map);

    // import rand: cargo add rand.
    // This would be added to Cargo.toml, under [dependencies]
    use rand::Rng;
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

     // To import whole package, 
     // eg use std::io::{self, Write};
     // eg global collections, use std::collections::*;
}
