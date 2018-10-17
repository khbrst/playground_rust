pub fn use_unwrap() {
  fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
  }

  let twenty = multiply("10", "2");
  println!("double is {}", twenty);

  // It will panic!
  // let tt = multiply("t", "2");
  // println!("double is {}", tt);
}

pub fn use_match_before_use_combinators() {
  use std::num::ParseIntError;

  // With the return type rewritten, we use pattern matching without `unwrap()`.
  fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
      Ok(first_number) => match second_number_str.parse::<i32>() {
        Ok(second_number) => Ok(first_number * second_number),
        Err(e) => Err(e),
      },
      Err(e) => Err(e),
    }
  }

  fn print(result: Result<i32, ParseIntError>) {
    match result {
      Ok(n) => println!("n is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  // This still presents a reasonable answer.
  let twenty = multiply("10", "2");
  print(twenty);

  // The following now provides a much more helpful error message.
  let tt = multiply("t", "2");
  print(tt);
}

pub fn use_combinators() {
  use std::num::ParseIntError;

  // As with `Option`, we can use combinators such as `map()`.
  // This function is otherwise identical to the one above and reads:
  // Modify n if the value is valid, otherwise pass on the error.
  fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
      second_number_str
        .parse::<i32>()
        .map(|second_number| first_number * second_number)
    })
  }

  fn print(result: Result<i32, ParseIntError>) {
    match result {
      Ok(n) => println!("n is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  // This still presents a reasonable answer.
  let twenty = multiply("10", "2");
  print(twenty);

  // The following now provides a much more helpful error message.
  let tt = multiply("t", "2");
  print(tt);
}

pub fn aliases() {
  use std::num::ParseIntError;

  // Define a generic alias for a `Result` with the error type `ParseIntError`.
  type AliasedResult<T> = Result<T, ParseIntError>;

  // Use the above alias to refer to our specific `Result` type.
  fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
      second_number_str
        .parse::<i32>()
        .map(|second_number| first_number * second_number)
    })
  }

  // Here, the alias again allows us to save some space.
  fn print(result: AliasedResult<i32>) {
    match result {
      Ok(n) => println!("n is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  print(multiply("10", "2"));
  print(multiply("t", "2"));
}

pub fn early_returns() {
  use std::num::ParseIntError;

  fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
      Ok(first_number) => first_number,
      Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
      Ok(second_number) => second_number,
      Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
  }

  fn print(result: Result<i32, ParseIntError>) {
    match result {
      Ok(n) => println!("n is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  print(multiply("10", "2"));
  print(multiply("t", "2"));
}

pub fn introducing_question_mark() {
  use std::num::ParseIntError;

  fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
  }

  fn print(result: Result<i32, ParseIntError>) {
    match result {
      Ok(n) => println!("n is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  print(multiply("10", "2"));
  print(multiply("t", "2"));
}

pub fn try_macro() {
  use std::num::ParseIntError;

  fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

    Ok(first_number * second_number)
  }

  fn print(result: Result<i32, ParseIntError>) {
    match result {
      Ok(n) => println!("n is {}", n),
      Err(e) => println!("Error: {}", e),
    }
  }

  print(multiply("10", "2"));
  print(multiply("t", "2"));
}
