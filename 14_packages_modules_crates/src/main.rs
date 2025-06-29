use crate::garden::vegetables::Asparagus;
mod garden;

fn main() {
    let plant = Asparagus {
        name: String::from("Green"),
        stalks: 10,
    };

    println!("Plant: {:?}", plant);
}
