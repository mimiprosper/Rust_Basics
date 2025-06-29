
// fn main() {
//     // array of characters of fixed sized
//     let arr = ['a', 'b', 'c', 'd', 'e'];
//     let range: &[char] = &arr[1..3];
//     println!("{:?}", range);

//     // slice of vector of integers
//     let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
//     let range = &vec[1..3];
//     println!("{:?}", range);

//     // slice of string
//     let s = String::from("Hello");
//     let range = &s[1..3];
//     println!("{:?}", range);

//     // slice short cut
//     let s = String::from("Hello");

//     let range = &s[1..];
//     println!("{:?}", range);

//     let range = &s[..3];
//     println!("{:?}", range);

//     let range = &s[..];
//     println!("{:?}", range);

//     let range = &s[0..3];
//     println!("{:?}", range);
// }

fn main() {    
    let s1  = String::from("hello world");
    let word = first_word(&s1);
    println!("the string is: {s1}");
    println!("The first word of the string is: {}", word);

    let s2 = String::from("great day");
    let word = first_word(&s2);
    println!("the string is: {s2}");
    println!("The first word of the string is: {}", word);
}

// function to get the first word of a string
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convert string to bytes

    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' { // if the byte is a space
        return &s[0..i]; // return the slice from 0 to i
      }
    }
    // if no space is found return the whole string
    &s[..] 
  }


