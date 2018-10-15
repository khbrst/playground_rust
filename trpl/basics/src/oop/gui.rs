pub trait Draw {
  fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    // code to actually draw a button
    println!("{:?}", self);
  }
}

#[derive(Debug)]
pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // code to actually draw a select box
    println!("{:?}", self);
  }
}

pub struct Screen {
  pub components: Vec<Box<Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
