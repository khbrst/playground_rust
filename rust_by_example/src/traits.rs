struct Sheep {
  naked: bool,
  name: &'static str,
}

trait Animal {
  // Static method signature; `Self` refers to the implementor type.
  fn new(name: &'static str) -> Self;

  // Instance method signatures; these will return a string.
  fn name(&self) -> &'static str;
  fn noise(&self) -> &'static str;

  // Traits can provide default method definitions.
  fn talk(&self) {
    println!("{} says {}", self.name(), self.noise());
  }
}

impl Sheep {
  fn is_naked(&self) -> bool {
    self.naked
  }

  fn shear(&mut self) {
    if self.is_naked() {
      // Implementor methods can use the implementor's trait methods.
      println!("{} is already naked...", self.name());
    } else {
      println!("{} gets a haircut!", self.name);

      self.naked = true;
    }
  }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
  // `Self` is the implementor type: `Sheep`.
  fn new(name: &'static str) -> Sheep {
    Sheep {
      name: name,
      naked: false,
    }
  }

  fn name(&self) -> &'static str {
    self.name
  }

  fn noise(&self) -> &'static str {
    if self.is_naked() {
      "baaaaah?"
    } else {
      "baaaaah!"
    }
  }

  // Default trait methods can be overridden.
  fn talk(&self) {
    // For example, we can add some quiet contemplation.
    println!("{} pauses briefly... {}", self.name, self.noise());
  }
}

pub fn basic() {
  // Type annotation is necessary in this case.
  let mut dolly: Sheep = Animal::new("Dolly");
  // TODO ^ Try removing the type annotations.

  dolly.talk();
  dolly.shear();
  dolly.talk();
}

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
  fn to_centimeters(&self) -> Centimeters {
    let &Inches(inches) = self;

    Centimeters(inches as f64 * 2.54)
  }
}

// `Seconds`, a tuple struct no additional attributes
struct Seconds(i32);

pub fn derive() {
  let _one_second = Seconds(1);

  // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
  //println!("One second looks like: {:?}", _one_second);
  // TODO ^ Try uncommenting this line

  // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
  //let _this_is_true = (_one_second == _one_second);
  // TODO ^ Try uncommenting this line

  let foot = Inches(12);

  println!("One foot equals {:?}", foot);

  let meter = Centimeters(100.0);

  let cmp = if foot.to_centimeters() < meter {
    "smaller"
  } else {
    "bigger"
  };

  println!("One foot is {} than one meter.", cmp);
}

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
  type Output = FooBar;

  fn add(self, _rhs: Bar) -> FooBar {
    println!("> Foo.add(Bar) was called");

    FooBar
  }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
  type Output = BarFoo;

  fn add(self, _rhs: Foo) -> BarFoo {
    println!("> Bar.add(Foo) was called");

    BarFoo
  }
}

pub fn operator_overloading() {
  println!("Foo + Bar = {:?}", Foo + Bar);
  println!("Bar + Foo = {:?}", Bar + Foo);
}

struct Droppable {
  name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
  fn drop(&mut self) {
    println!("> Dropping {}", self.name);
  }
}

pub fn do_drop() {
  let _a = Droppable { name: "a" };

  // block A
  {
    let _b = Droppable { name: "b" };

    // block B
    {
      let _c = Droppable { name: "c" };
      let _d = Droppable { name: "d" };

      println!("Exiting block B");
    }
    println!("Just exited block B");

    println!("Exiting block A");
  }
  println!("Just exited block A");

  // Variable can be manually dropped using the `drop` function
  drop(_a);
  // TODO ^ Try commenting this line

  println!("end of the main function");

  // `_a` *won't* be `drop`ed again here, because it already has been
  // (manually) `drop`ed
}

struct Fibonacci {
  curr: u32,
  next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
  type Item = u32;

  // Here, we define the sequence using `.curr` and `.next`.
  // The return type is `Option<T>`:
  //     * When the `Iterator` is finished, `None` is returned.
  //     * Otherwise, the next value is wrapped in `Some` and returned.
  fn next(&mut self) -> Option<u32> {
    let new_next = self.curr + self.next;

    self.curr = self.next;
    self.next = new_next;

    // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
    // will never return `None`, and `Some` is always returned.
    Some(self.curr)
  }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
  Fibonacci { curr: 1, next: 1 }
}

pub fn iterators() {
  // `0..3` is an `Iterator` that generates: 0, 1, and 2.
  let mut sequence = 0..3;

  println!("Four consecutive `next` calls on 0..3");
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());

  // `for` works through an `Iterator` until it returns `None`.
  // Each `Some` value is unwrapped and bound to a variable (here, `i`).
  println!("Iterate through 0..3 using `for`");
  for i in 0..3 {
    println!("> {}", i);
  }

  // The `take(n)` method reduces an `Iterator` to its first `n` terms.
  println!("The first four terms of the Fibonacci sequence are: ");
  for i in fibonacci().take(4) {
    println!("> {}", i);
  }

  // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
  println!("The next four terms of the Fibonacci sequence are: ");
  for i in fibonacci().skip(4).take(4) {
    println!("> {}", i);
  }

  let array = [1u32, 3, 3, 7];

  // The `iter` method produces an `Iterator` over an array/slice.
  println!("Iterate the following array {:?}", &array);
  for i in array.iter() {
    println!("> {}", i);
  }
}

// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Nil;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

pub fn clone() {
  // Instantiate `Nil`
  let nil = Nil;
  // Copy `Nil`, there are no resources to move
  let copied_nil = nil;

  // Both `Nil`s can be used independently
  println!("original: {:?}", nil);
  println!("copy: {:?}", copied_nil);

  // Instantiate `Pair`
  let pair = Pair(Box::new(1), Box::new(2));
  println!("original: {:?}", pair);

  // Copy `pair` into `moved_pair`, moves resources
  let moved_pair = pair;
  println!("copy: {:?}", moved_pair);

  // Error! `pair` has lost its resources
  //println!("original: {:?}", pair);
  // TODO ^ Try uncommenting this line

  // Clone `moved_pair` into `cloned_pair` (resources are included)
  let cloned_pair = moved_pair.clone();
  // Drop the original pair using std::mem::drop
  drop(moved_pair);

  // Error! `moved_pair` has been dropped
  //println!("copy: {:?}", moved_pair);
  // TODO ^ Try uncommenting this line

  // The result from .clone() can still be used!
  println!("clone: {:?}", cloned_pair);
}
