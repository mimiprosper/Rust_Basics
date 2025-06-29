fn main() {
  let x = 5; // statement
  println!("The value of x is: {}", x);

  // expression
  let y = {
    let x = 7;
    x + 3
  };
  println!("The value of y is: {}", y);

  // return values fron function
  fn sum(x: i32, y: i32) -> i32 {
    x + y
  }

  let result = sum(3, 5);
  println!("The value of result is: {}", result);

}

