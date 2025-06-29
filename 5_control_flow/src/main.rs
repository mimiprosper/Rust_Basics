// https://doc.rust-lang.org/book/ch03-05-control-flow.html

// fn main() {
// (if statement)
//     let num = 21;

//     if num % 2 == 0 {
//         println!("{} is even", num);
//     } else {
//         println!("{} is odd", num);
//     }
//     if num > 10 {
//         println!("{} is greater than 10", num);
//     }else {
//         println!("{} is less than 10", num);
//     }
// }

// fn main() {
//     // (match statement)
//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter,
//     }

//    fn value_in_cents(coin: Coin) -> u8 {
//        match coin {
//            Coin::Penny => 1,
//            Coin::Nickel => 5,
//            Coin::Dime => 10,
//            Coin::Quarter => 25,
//        }
//    }

//    let coin = Coin::Nickel;
//    println!("{} cents", value_in_cents(coin));

//    let coin = Coin::Quarter;
//    println!("{} cents", value_in_cents(coin));

//    let coin = Coin::Dime;
//    println!("{} cents", value_in_cents(coin));

//    let coin = Coin::Penny;
//    println!("{} cents", value_in_cents(coin));
// }

// fn main() {
//     // (while loop)
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     // println!("The result is {result}" );
//     println!("The result is {}", result );
// }

fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    let s = "hello world";
    for c in s.chars() {
        println!("{}", c);
    }

    for n in 1..5 {
        println!("{}!", n);
    }
}

// fizzbuzz
// fn main() {
//     for n in 1..100 {
//         if n % 3 == 0 && n % 5 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//     }
// }