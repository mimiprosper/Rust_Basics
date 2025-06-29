// https://doc.rust-lang.org/book/
fn main() {
    // let mut v1: Vec<i32> = Vec::new(); // empty vector
    // let mut v = vec![1, 2, 3]; // non-empty vector

    // println!("first empty vector, V1: {:?}", v1);
    // println!("first non-empty vector, V: {:?}", v);

    // v1.push(7);
    // v1.push(8);
    // println!("updated empty vector, V1: {:?}", v1);

    // v.push(4); // add element
    // v.push(5); // add element
    // v.push(6); // add element
    // println!("updated non-empty vector, V: {:?}", v);

    let v3 = ["a", "b", "c", "d", "e", "f"];

    let index4 = v3.get(4);
    // println!("This is the {:?}", index4);

    // match index4 {
    //     Some(index4) => println!("This is index 4: {}", index4),
    //     None => println!("This is index no index 4"),
    // }

    let index5: &str = &v3[5];
    // println!("This is index 5: {:?}", index5);

    // pushinh vector
    let mut v4 = Vec::new();
    v4.push(1);
    v4.push(2);
    v4.push(3);
    v4.push(4);
    v4.push(5);
    v4.push(6);
    // println!("This is the vector v4: {:?} ", v4);

    // interating vector
    for i in &v4{
        // println!("This is the vector v4: {:?} ", i);
    }

    v4.pop();
    // println!("This is the vector v4 after pop: {:?} ", v4);

    // store multiple types in a vector using enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("This is the vector row: {:?} ", row);

}