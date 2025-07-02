//enums are user defined types

#[derive(Debug)]
enum IPAddressKind {
   V4(String),
   V6(String)
}

// implementation with enums
#[derive(Debug)]
enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// method for enums
impl Messages {
    fn call(&self) {
        println!("Message: {:?}", self);
    }
}

fn main() {
    // instances of the above enums
    let quit = Messages::Quit;
    let move_cursor = Messages::Move { x: 10, y: 20 };
    let write = Messages::Write(String::from("hello"));
    let change_color = Messages::ChangeColor(10, 20, 30);

    quit.call();
    move_cursor.call();
    write.call();
    change_color.call();

    let home = IPAddressKind::V4(String::from("127.0.0.1"));
    let loopback = IPAddressKind::V6(String::from("::1"));
    println!("home = {:?}, loopback = {:?}", home, loopback);
}