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
