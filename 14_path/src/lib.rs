// create a library: cargo new <folder name> --lib

#![allow(unused)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn define_order() {}

mod back_of_house {
    pub enum Appetizer { // public enums
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::define_order();
    }
    fn cook_order() {}
}

mod inside_the_kitchen {
    pub struct Breakfast {  // public struct
        pub toast: String,
        seasonal_fruit: String,
    }
   
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_canteen() {
    let mut meal = inside_the_kitchen::Breakfast::summer("Rye");
    println!("I'd like {}, toast please", meal.toast);

    meal.toast = String::from("Wheat");
    println!("I'd like {}, toast please", meal.toast);
}

fn eat_at_cateria() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}