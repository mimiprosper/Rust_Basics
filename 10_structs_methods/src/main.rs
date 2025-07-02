// Rectangle struct
struct Rectangle {
    width: u32,
    height: u32,
}

// impl block is used to define methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method with same name of a field
    fn width(&self) -> bool {
        self.width > 0
    }

    // getter for height
    fn height(&self) -> u32 {
        self.height
    }

    // check if rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
            || self.width > other.height && self.height > other.width
    }

    // associated function to define a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


/// Create three instances of Rectangle and a square
/// and print if rect1 can hold the other two
/// and print the area of the square
/// and print the area, width and height of rect1

fn main() {
    // instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // instance of Rectangle
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    // instance of Rectangle
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // create a square
    let square = Rectangle::square(20);
    println!("The area of the square is {} square pixels", square.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels, width > 0: {}, height: {}",
        rect1.area(),
        rect1.width(),
        rect1.height()
    );
}
