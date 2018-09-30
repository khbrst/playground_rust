mod gui {
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
}

use self::gui::{Button, Screen, SelectBox};

pub fn implementing_the_trait() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();
}
