pub fn default_generic_type_parameter_and_operator_overloading() {
  use std::ops::Add;

  #[derive(Debug, PartialEq)]
  struct Point {
    x: i32,
    y: i32,
  }

  impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
      Point {
        x: self.x + other.x,
        y: self.y + other.y,
      }
    }
  }

  struct Millimeters(u32);
  struct Meters(u32);

  impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + (other.0 * 1000))
    }
  }

  assert_eq!(
    Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    Point { x: 3, y: 3 }
  );
}

pub fn use_fully_qualified_syntax() {
  trait Animal {
    fn baby_name() -> String;
  }

  struct Dog;

  impl Dog {
    fn baby_name() -> String {
      String::from("Spot")
    }
  }

  impl Animal for Dog {
    fn baby_name() -> String {
      String::from("puppy")
    }
  }

  println!("A baby dog is called a {}", Dog::baby_name());
  // error[E0283]: type annotations required: cannot resolve `_: Animal`
  // println!("A baby dog is called a {}", Animal::baby_name());
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

pub fn use_super_trait() {
  use std::fmt;

  trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
      let output = self.to_string();
      let len = output.len();
      println!("{}", "*".repeat(len + 4));
      println!("*{}*", " ".repeat(len + 2));
      println!("* {} *", output);
      println!("*{}*", " ".repeat(len + 2));
      println!("{}", "*".repeat(len + 4));
    }
  }

  struct Point {
    x: i32,
    y: i32,
  }

  impl OutlinePrint for Point {}

  impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
    }
  }

  let p = Point { x: 0, y: 12 };
  p.outline_print();
}

pub fn use_newtype_pattern() {
  use std::fmt;

  struct Wrapper(Vec<String>);

  impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "[{}]", self.0.join(", "))
    }
  }

  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}
