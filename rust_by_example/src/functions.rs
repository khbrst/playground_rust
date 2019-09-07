// Unlike C/C++, there's no restriction on the order of function definitions
pub fn call_functions() {
  // We can use this function here, and define it somewhere later
  fizzbuzz_to(100);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
  // Corner case, early return
  if rhs == 0 {
    return false;
  }

  // This is an expression, the `return` keyword is not necessary here
  lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
  if is_divisible_by(n, 15) {
    println!("fizzbuzz");
  } else if is_divisible_by(n, 3) {
    println!("fizz");
  } else if is_divisible_by(n, 5) {
    println!("buzz");
  } else {
    println!("{}", n);
  }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
  for n in 1..n + 1 {
    fizzbuzz(n);
  }
}

struct Point {
  x: f64,
  y: f64,
}

// Implementation block, all `Point` methods go in here
impl Point {
  // This is a static method
  // Static methods don't need to be called by an instance
  // These methods are generally used as constructors
  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }

  // Another static method, taking two arguments:
  fn new(x: f64, y: f64) -> Point {
    Point { x: x, y: y }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  // This is an instance method
  // `&self` is sugar for `self: &Self`, where `Self` is the type of the
  // caller object. In this case `Self` = `Rectangle`
  fn area(&self) -> f64 {
    // `self` gives access to the struct fields via the dot operator
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    // `abs` is a `f64` method that returns the absolute value of the
    // caller
    ((x1 - x2) * (y1 - y2)).abs()
  }

  fn perimeter(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
  }

  // This method requires the caller object to be mutable
  // `&mut self` desugars to `self: &mut Self`
  fn translate(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p2.x += x;

    self.p1.y += y;
    self.p2.y += y;
  }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
  // This method "consumes" the resources of the caller object
  // `self` desugars to `self: Self`
  fn destroy(self) {
    // Destructure `self`
    let Pair(first, second) = self;

    println!("Destroying Pair({}, {})", first, second);

    // `first` and `second` go out of scope and get freed
  }
}

pub fn call_methods() {
  let rectangle = Rectangle {
    // Static methods are called using double colons
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };

  // Instance methods are called using the dot operator
  // Note that the first argument `&self` is implicitly passed, i.e.
  // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
  println!("Rectangle perimeter: {}", rectangle.perimeter());
  println!("Rectangle area: {}", rectangle.area());

  let mut square = Rectangle {
    p1: Point::origin(),
    p2: Point::new(1.0, 1.0),
  };

  // Error! `rectangle` is immutable, but this method requires a mutable
  // object
  //rectangle.translate(1.0, 0.0);
  // TODO ^ Try uncommenting this line

  // Okay! Mutable objects can call mutable methods
  square.translate(1.0, 1.0);

  let pair = Pair(Box::new(1), Box::new(2));

  pair.destroy();

  // Error! Previous `destroy` call "consumed" `pair`
  //pair.destroy();
  // TODO ^ Try uncommenting this line
}

pub fn closure() {
  // Increment via closures and functions.
  fn function(i: i32) -> i32 {
    i + 1
  }

  // Closures are anonymous, here we are binding them to references
  // Annotation is identical to function annotation but is optional
  // as are the `{}` wrapping the body. These nameless functions
  // are assigned to appropriately named variables.
  let closure_annotated = |i: i32| -> i32 { i + 1 };
  let closure_inferred = |i| i + 1;

  let i = 1;
  // Call the function and closures.
  println!("function: {}", function(i));
  println!("closure_annotated: {}", closure_annotated(i));
  println!("closure_inferred: {}", closure_inferred(i));

  // A closure taking no arguments which returns an `i32`.
  // The return type is inferred.
  let one = || 1;
  println!("closure returning one: {}", one());
}

pub fn closure_capturing() {
  use std::mem;

  let color = "green";

  // A closure to print `color` which immediately borrows (`&`)
  // `color` and stores the borrow and closure in the `print`
  // variable. It will remain borrowed until `print` goes out of
  // scope. `println!` only requires `by reference` so it doesn't
  // impose anything more restrictive.
  let print = || println!("`color`: {}", color);

  // Call the closure using the borrow.
  print();
  print();

  let mut count = 0;

  // A closure to increment `count` could take either `&mut count`
  // or `count` but `&mut count` is less restrictive so it takes
  // that. Immediately borrows `count`.
  //
  // A `mut` is required on `inc` because a `&mut` is stored inside.
  // Thus, calling the closure mutates the closure which requires
  // a `mut`.
  let mut inc = || {
    count += 1;
    println!("`count`: {}", count);
  };

  // Call the closure.
  inc();
  inc();

  //let _reborrow = &mut count;
  // ^ TODO: try uncommenting this line.

  // A non-copy type.
  let movable = Box::new(3);

  // `mem::drop` requires `T` so this must take by value. A copy type
  // would copy into the closure leaving the original untouched.
  // A non-copy must move and so `movable` immediately moves into
  // the closure.
  let consume = || {
    println!("`movable`: {:?}", movable);
    mem::drop(movable);
  };

  // `consume` consumes the variable so this can only be called once.
  consume();
  //consume();
  // ^ TODO: Try uncommenting this line.
}

pub fn closure_capturing_use_move() {
  // `Vec` has non-copy semantics.
  let haystack = vec![1, 2, 3];

  let contains = move |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));

  // `println!("There're {} elements in vec", haystack.len());`
  // ^ Uncommenting above line will result in compile-time error
  // because borrow checker doesn't allow re-using variable after it
  // has been moved.

  // Removing `move` from closure's signature will cause closure
  // to borrow _haystack_ variable immutably, hence _haystack_ is still
  // available and uncommenting above line will not cause an error.
}

pub fn closure_as_input_parameter() {
  use std::mem;

  // A function which takes a closure as an argument and calls it.
  fn apply<F>(f: F)
  where
    // The closure takes no input and returns nothing.
    F: FnOnce(),
  {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
  }

  // A function which takes a closure and returns an `i32`.
  fn apply_to_3<F>(f: F) -> i32
  where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32,
  {
    f(3)
  }

  let greeting = "hello";
  // A non-copy type.
  // `to_owned` creates owned data from borrowed one
  let mut farewell = "goodbye".to_owned();

  // Capture 2 variables: `greeting` by reference and
  // `farewell` by value.
  let diary = || {
    // `greeting` is by reference: requires `Fn`.
    println!("I said {}.", greeting);

    // Mutation forces `farewell` to be captured by
    // mutable reference. Now requires `FnMut`.
    farewell.push_str("!!!");
    println!("Then I screamed {}.", farewell);
    println!("Now I can sleep. zzzzz");

    // Manually calling drop forces `farewell` to
    // be captured by value. Now requires `FnOnce`.
    mem::drop(farewell);
  };

  // Call the function which applies the closure.
  apply(diary);

  // `double` satisfies `apply_to_3`'s trait bound
  let double = |x| 2 * x;

  println!("3 doubled: {}", apply_to_3(double));
}

pub fn closure_type_anonymity() {
  // `F` must implement `Fn` for a closure which takes no
  // inputs and returns nothing - exactly what is required
  // for `print`.
  fn apply<F>(f: F)
  where
    F: Fn(),
  {
    f();
  }

  let x = 7;

  // Capture `x` into an anonymous type and implement
  // `Fn` for it. Store it in `print`.
  let print = || println!("{}", x);

  apply(print);
}

pub fn closure_input_function() {
  // Define a function which takes a generic `F` argument
  // bounded by `Fn`, and calls it
  fn call_me<F: Fn()>(f: F) {
    f();
  }

  // Define a wrapper function satisfying the `Fn` bound
  fn function() {
    println!("I'm a function!");
  }

  // Define a closure satisfying the `Fn` bound
  let closure = || println!("I'm a closure!");

  call_me(closure);
  call_me(function);
}

pub fn closure_as_output_parameter() {
  fn create_fn() -> Box<dyn Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
  }

  fn create_fnmut() -> Box<dyn FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}", text))
  }

  let fn_plain = create_fn();
  let mut fn_mut = create_fnmut();

  fn_plain();
  fn_mut();
}

pub fn closure_iterator_any() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  // `iter()` for vecs yields `&i32`. Destructure to `i32`.
  println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
  // `into_iter()` for vecs yields `i32`. No destructuring required.
  println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  // `iter()` for arrays yields `&i32`.
  println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
  // `into_iter()` for arrays unusually yields `&i32`.
  println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

pub fn closure_iterator_find() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  // `iter()` for vecs yields `&i32`.
  let mut iter = vec1.iter();
  // `into_iter()` for vecs yields `i32`.
  let mut into_iter = vec2.into_iter();

  // A reference to what is yielded is `&&i32`. Destructure to `i32`.
  println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
  // A reference to what is yielded is `&i32`. Destructure to `i32`.
  println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

  let array1 = [1, 2, 3];
  let array2 = [4, 5, 6];

  // `iter()` for arrays yields `&i32`
  println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
  // `into_iter()` for arrays unusually yields `&i32`
  println!(
    "Find 2 in array2: {:?}",
    array2.into_iter().find(|&&x| x == 2)
  );
}

pub fn high_order_functions() {
  fn is_odd(n: u32) -> bool {
    n % 2 == 1
  }

  println!("Find the sum of all the squared odd numbers under 1000");
  let upper = 1000;

  // Imperative approach
  // Declare accumulator variable
  let mut acc = 0;
  // Iterate: 0, 1, 2, ... to infinity
  for n in 0.. {
    // Square the number
    let n_squared = n * n;

    if n_squared >= upper {
      // Break loop if exceeded the upper limit
      break;
    } else if is_odd(n_squared) {
      // Accumulate value, if it's odd
      acc += n_squared;
    }
  }
  println!("imperative style: {}", acc);

  // Functional approach
  let sum_of_squared_odd_numbers: u32 = (0..).map(|n| n * n)                             // All natural numbers squared
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
             .fold(0, |acc, n_squared| acc + n_squared); // Sum them
  println!("functional style: {}", sum_of_squared_odd_numbers);
}

pub fn diverging_functions() {
  fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
      // Notice that the return type of this match expression must be u32
      // because of the type of the "addition" variable.
      let addition: u32 = match i % 2 == 1 {
        // The "i" variable is of type u32, which is perfectly fine.
        true => i,
        // On the other hand, the "continue" expression does not return
        // u32, but it is still fine, because it never returns and therefore
        // does not violate the type requirements of the match expression.
        false => continue,
      };
      acc += addition;
    }
    acc
  }
  println!(
    "Sum of odd numbers up to 9 (excluding): {}",
    sum_odd_numbers(9)
  );
}
