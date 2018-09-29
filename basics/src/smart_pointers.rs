pub fn box_save_to_heap() {
  let b = Box::new(5);
  println!("b = {}", b);
}

enum List {
  Cons(i32, Box<List>),
  Nil,
}

use self::List::{Cons, Nil};

pub fn implement_cons_list_use_box() {
  #[allow(unused_variables)]
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

pub fn implement_custom_smart_pointer() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}
