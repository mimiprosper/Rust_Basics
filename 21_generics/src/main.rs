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
/// Returns the new element of the pair.
    fn new(&self) -> &T {
        &self.x
        
    } 

/// Returns the old element of the pair.
    fn old(&self) -> &U {
        &self.y
    }
}


/// This is the main function.
///
/// It demonstrates the use of generics in Rust.
/// It first creates a vector of numbers and characters,
/// then finds the largest element in the vector,
/// and prints it out.
/// It also creates a struct `Pair` with two generic types,
/// and a method `new` that returns a reference to the first type,
/// and a method `old` that returns a reference to the second type.
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

