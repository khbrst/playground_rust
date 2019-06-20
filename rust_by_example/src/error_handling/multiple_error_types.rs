pub fn before_handling() {
  fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
  }

  let numbers = vec!["42", "93", "18"];
  let _empty: std::vec::Vec<&str> = vec![];
  let _strings = vec!["tofu", "93", "18"];

  println!("The first doubled is {}", double_first(numbers));

  // println!("The first doubled is {}", double_first(_empty));
  // Error 1: the input vector is empty

  // println!("The first doubled is {}", double_first(_strings));
  // Error 2: the element doesn't parse to a number
}

pub fn pulling_results_out_of_options() {
  use std::num::ParseIntError;

  fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
  }

  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  println!("The first doubled is {:?}", double_first(numbers));

  println!("The first doubled is {:?}", double_first(empty));
  // Error 1: the input vector is empty

  println!("The first doubled is {:?}", double_first(strings));
  // Error 2: the element doesn't parse to a number
}

pub fn pulling_results_out_of_options_with_combinators() {
  use std::num::ParseIntError;

  fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    let opt = opt.map_or(Ok(None), |r| r.map(Some))?;

    Ok(opt)
  }

  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  println!("The first doubled is {:?}", double_first(numbers));
  println!("The first doubled is {:?}", double_first(empty));
  println!("The first doubled is {:?}", double_first(strings));
}

pub fn defining_an_error_type() {
  use std::error;
  use std::fmt;

  type Result<T> = std::result::Result<T, DoubleError>;

  #[derive(Debug, Clone)]
  // Define our error types. These may be customized for our error handling cases.
  // Now we will be able to write our own errors, defer to an underlying error
  // implementation, or do something in between.
  struct DoubleError;

  // Generation of an error is completely separate from how it is displayed.
  // There's no need to be concerned about cluttering complex logic with the display style.
  //
  // Note that we don't store any extra info about the errors. This means we can't state
  // which string failed to parse without modifying our types to carry that information.
  impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "invalid first item to double")
    }
  }

  // This is important for other errors to wrap this one.
  impl error::Error for DoubleError {
    fn description(&self) -> &str {
      "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
      // Generic error, underlying cause isn't tracked.
      None
    }
  }

  fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // Change the error to our new type.
       .ok_or(DoubleError)
       .and_then(|s| s.parse::<i32>()
            // Update to the new error type here also.
            .map_err(|_| DoubleError)
            .map(|i| 2 * i))
  }

  fn print(result: Result<i32>) {
    match result {
      Ok(n) => println!("The first doubled is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  print(double_first(numbers));
  print(double_first(empty));
  print(double_first(strings));
}

pub fn boxing_errors() {
  use std::error;
  use std::fmt;

  // Change the alias to `Box<error::Error>`.
  type Result<T> = std::result::Result<T, Box<error::Error>>;

  #[derive(Debug, Clone)]
  struct EmptyVec;

  impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "invalid first item to double")
    }
  }

  impl error::Error for EmptyVec {
    fn description(&self) -> &str {
      "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
      // Generic error, underlying cause isn't tracked.
      None
    }
  }

  fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       .ok_or_else(|| EmptyVec.into())  // Converts to Box
       .and_then(|s| s.parse::<i32>()
            .map_err(|e| e.into())  // Converts to Box
            .map(|i| 2 * i))
  }

  fn print(result: Result<i32>) {
    match result {
      Ok(n) => println!("The first doubled is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  print(double_first(numbers));
  print(double_first(empty));
  print(double_first(strings));
}

pub fn other_uses_of_question_mark() {
  use std::error;
  use std::fmt;

  // Change the alias to `Box<error::Error>`.
  type Result<T> = std::result::Result<T, Box<error::Error>>;

  #[derive(Debug)]
  struct EmptyVec;

  impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "invalid first item to double")
    }
  }

  impl error::Error for EmptyVec {
    fn description(&self) -> &str {
      "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
      // Generic error, underlying cause isn't tracked.
      None
    }
  }

  // The same structure as before but rather than chain all `Results`
  // and `Options` along, we `?` to get the inner value out immediately.
  fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
  }

  fn print(result: Result<i32>) {
    match result {
      Ok(n) => println!("The first doubled is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  print(double_first(numbers));
  print(double_first(empty));
  print(double_first(strings));
}

pub fn wrapping_errors() {
  use std::error;
  use std::fmt;
  use std::num::ParseIntError;

  type Result<T> = std::result::Result<T, DoubleError>;

  #[derive(Debug)]
  enum DoubleError {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
  }

  impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
        DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
        // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
        DoubleError::Parse(ref e) => e.fmt(f),
      }
    }
  }

  impl error::Error for DoubleError {
    fn description(&self) -> &str {
      match *self {
        DoubleError::EmptyVec => "empty vectors not allowed",
        // This already impls `Error`, so defer to its own implementation.
        DoubleError::Parse(ref e) => e.description(),
      }
    }

    fn cause(&self) -> Option<&error::Error> {
      match *self {
        DoubleError::EmptyVec => None,
        // The cause is the underlying implementation error type. Is implicitly
        // cast to the trait object `&error::Error`. This works because the
        // underlying type already implements the `Error` trait.
        DoubleError::Parse(ref e) => Some(e),
      }
    }
  }

  // Implement the conversion from `ParseIntError` to `DoubleError`.
  // This will be automatically called by `?` if a `ParseIntError`
  // needs to be converted into a `DoubleError`.
  impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
      DoubleError::Parse(err)
    }
  }

  fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
  }

  fn print(result: Result<i32>) {
    match result {
      Ok(n) => println!("The first doubled is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  let numbers = vec!["42", "93", "18"];
  let empty = vec![];
  let strings = vec!["tofu", "93", "18"];

  print(double_first(numbers));
  print(double_first(empty));
  print(double_first(strings));
}
