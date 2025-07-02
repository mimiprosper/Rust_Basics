// match example 1

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Rarity),
}

#[allow(dead_code)]
enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Unique
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,    
        Coin::Quarter(rarity) => 25
    }

}

// match statemen example 2
fn main() {
      let coin = Coin::Quarter(Rarity::Common);
    println!("Value in cents: {}", value_in_cents(coin));

    let roll_dice = 5;

    let result = match roll_dice {
        3 => 3,
        4 => 4,
        5 => 5, 
        6 => 6,
        _ => 0,
    };
    println!("result: {}", result);
}