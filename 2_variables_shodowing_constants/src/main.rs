fn main() {
    // mut keyword make the value to change but the type would not change
    // let mut x = 5;
    // println!("The Result: {}", x);

    // x = 6;
    // println!("The Result: {}", x);

    // x = "hello";
    // println!("Print {}", x);



    shadowing
    let x = 5;
    println!("The Result: {}", x);

    // inner scope
    {
        let x = 8;
        println!("The Result: {}", x);
    }

    let x = x + 2;
    println!("The Result: {}", x);

    let x = "hello";
    println!("Print {}", x);



    // constants
    const MAX_POINTS: u32 = 100_000;
    println!("The Result: {}", MAX_POINTS);
}
