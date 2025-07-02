#![allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}

#[derive(Debug)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}


/// This function demonstrates the use of `if let` statements to concisely
/// handle the Option enum and to destructure enums.
///
/// It first checks if the value of `config_max` is `Some` and prints the
/// configured maximum value.
///
/// Then, it checks if the value of `coin` is `Coin::Quarter` and prints
/// the rarity of the quarter.

fn main() {
    let config_max = Some(100);
    // use if-let statement to check if value is Some
    if let Some(max) = config_max {
       println!("The maximum is configured to be {}", max);
    }

   let coin = Coin::Quarter(Rarity::Epic);
   // let coin = Coin::Penny;

    if let Coin::Quarter(rarity) = coin {
        println!("The coin is a {:?} quarter", rarity);
    } else {
        println!("The coin is not a quarter");
    }
}
