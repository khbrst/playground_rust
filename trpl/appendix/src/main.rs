#[macro_use]
extern crate appendix;
extern crate hello_macro;

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
  fn hello_macro() {
    println!("Hello, Macro! My name is Pancakes!");
  }
}

fn main() {
  let declarative_macro = my_vec!(1, 2, 3);
  println!("{:?}", declarative_macro);

  Pancakes::hello_macro();
}
