use std::collections::HashMap;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }
    }
}

mod eat_at_restaurant{
    use crate::front_of_house::hosting;
    pub fn serve_order() {
        hosting::add_to_waitlist();
    }
}

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
