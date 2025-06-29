#![allow(dead_code)]
// Generics in functions: A function that works for any data type
// Function to get the highest value in a list
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U
}

// Generics in enums
enum Option<T> {
    Some(T),
    None
}

// enum with multiple genric types
enum Result<T, E> {
    Ok(T),
    Err(E)
}

// Generics in struts
// Generics in methods
struct Pair<T, U> {
    x: T,
    y: U
}

// method implementation
impl<T, U> Pair<T, U> {
    fn new(&self) -> &T {
        &self.x
        
    } 
    fn old(&self) -> &U {
        &self.y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let char = Point { x: 'a', y: 'b' };

    println!("Integer: ({}, {})", integer.x, integer.y);
    println!("Float: ({}, {})", float.x, float.y);
    println!("Char: ({}, {})", char.x, char.y);

    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("Integer Float: ({}, {})", integer_and_float.x, integer_and_float.y);

    let answer = Pair { x: 5, y: 10.4 };
    println!("Pair: ({}, {})", answer.new(), answer.old());

}

