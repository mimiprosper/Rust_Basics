
/// This is the main function
///
/// It demonstrates the use of the `mut` keyword, shadowing, and constants.
///
/// # Examples
///
/// 

fn main() {
    
    // shadowing
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
