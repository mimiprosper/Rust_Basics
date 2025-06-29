// // ownership & functions
// fn main() {
//     let i = 5;
//     call_int(i);
//     println!("AFTER CALLING THE FUNCTION THE VALUE OF i: {}", i);

//     let s = String::from("hello");
//     call_string(s);
//     // println!("AFTER CALLING THE FUNCTION THE VALUE OF s: {}", s);
// }

// // call call_int function
// fn call_int(i: i32) {
//     println!("i: {}", i);
// }

// fn call_string(s: String) {
//     println!("s: {}", s);
// }

fn main() {
    let s1 = String::from("hello");
    let len= calculate_length(s1);
    println!("The length of {}", len);
}

fn calculate_length(s: String) -> usize {
    let string_length = s.len();
    string_length
}

