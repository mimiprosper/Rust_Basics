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

fn main() {
    // let config_max = Some(100);

    // // use if-let statement to check if value is Some
    // if let Some(max) = config_max {
    //    println!("The maximum is configured to be {}", max);
    // }

   let coin = Coin::Quarter(Rarity::Epic);
   // let coin = Coin::Penny;

    if let Coin::Quarter(rarity) = coin {
        println!("The coin is a {:?} quarter", rarity);
    } else {
        println!("The coin is not a quarter");
    }
}
