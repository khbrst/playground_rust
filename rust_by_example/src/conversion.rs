use std::convert::From;

#[derive(Debug)]
struct Number {
  value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}

pub fn from_and_into() {
  let num = Number::from(30);
  println!("My number is {:?}", num);

  let int = 5;
  // Try removing the type declaration
  let num: Number = int.into();
  println!("My number is {:?}", num);
}

use std::string::ToString;

struct Circle {
  radius: i32,
}

impl ToString for Circle {
  fn to_string(&self) -> String {
    format!("Circle of radius {:?}", self.radius)
  }
}

pub fn to_string() {
  let circle = Circle { radius: 6 };
  println!("{}", circle.to_string());
}

pub fn parsing_a_string() {
  let parsed: i32 = "5".parse().unwrap();
  // `turbofish` syntax is `::<>`
  let turbo_parsed = "10".parse::<i32>().unwrap();

  let sum = parsed + turbo_parsed;
  println!{"Sum: {:?}", sum};
}
