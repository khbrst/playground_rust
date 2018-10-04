struct Point {
  x: i32,
  y: i32,
}

pub fn destructure_structs() {
  let p = Point { x: 0, y: 7 };

  let Point { x, y } = p;
  assert_eq!(0, x);
  assert_eq!(7, y);
}

pub fn destructure_structs_in_match() {
  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}

#[allow(dead_code)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

pub fn destructure_enums() {
  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::Quit => println!("The Quit variant has no data to destructure."),
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
  }
}

#[allow(unused_variables)]
pub fn destructure_reference() {
  let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
  ];

  let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
}

#[allow(unused_variables)]
pub fn destructure_structs_and_tuples() {
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

pub fn create_reference() {
  let robot_name = Some(String::from("Bors"));

  match robot_name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
  }

  println!("robot_name is: {:?}", robot_name);
}

pub fn create_mutable_reference() {
  let mut robot_name = Some(String::from("Bors"));

  match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
  }

  println!("robot_name is: {:?}", robot_name);
}

pub fn match_guards() {
  let num = Some(4);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {:?}", n),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);

  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }
}

pub fn at_operator_bindings() {
  enum Message {
    Hello { id: i32 },
  }

  let msg = Message::Hello { id: 5 };

  match msg {
    Message::Hello {
      id: id_variable @ 3...7,
    } => println!("Found an id in range: {}", id_variable),
    Message::Hello { id: 10...12 } => println!("Found an id in another range"),
    Message::Hello { id } => println!("Found some other id: {}", id),
  }
}
