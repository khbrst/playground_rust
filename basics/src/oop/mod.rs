mod blog_trait;
mod blog_type;
mod gui;

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

pub fn implementing_state_design_pattern_as_trait() {
  use self::blog_trait::Post;

  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}

pub fn implementing_state_design_pattern_as_type() {
  use self::blog_type::Post;

  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");

  let post = post.request_review();

  let post = post.approve();

  assert_eq!("I ate a salad for lunch today", post.content());
}
