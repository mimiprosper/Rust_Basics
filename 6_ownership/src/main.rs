// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

fn main() {
    // let a1 = String::from("hello");
    // // s1.push_str(", world");
    // let a2 = a1.clone();
    // println!("a1 = {}, a2 = {}", a1, a2);

    // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward

    // do stuff with s
} // this scope is now over (out fo scope), and s is no longer valid
