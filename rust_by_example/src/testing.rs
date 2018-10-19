/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `rust_by_example::testing` crate:
///
/// ```
/// let result = rust_by_example::testing::add(2, 3);
/// assert_eq!(result, 5);
/// ```
#[allow(dead_code)]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = rust_by_example::testing::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// rust_by_example::testing::div(10, 0);
/// ```
#[allow(dead_code)]
pub fn div(a: i32, b: i32) -> i32 {
  if b == 0 {
    panic!("Divide-by-zero error");
  }

  a / b
}

/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compileable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = rust_by_example::testing::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #    try_main().unwrap(); // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// # }
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
  if b == 0 {
    Err(String::from("Divide-by-zero"))
  } else {
    Ok(a / b)
  }
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
  a - b
}

#[allow(dead_code)]
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
  if b == 0 {
    panic!("Divide-by-zero error");
  } else if a < b {
    panic!("Divide result is zero");
  }
  a / b
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_add() {
    assert_eq!(add(1, 2), 3);
  }

  // #[test]
  #[allow(dead_code)]
  fn test_bad_add() {
    // This assert would fire and test will fail.
    // Please note, that private functions can be tested too!
    assert_eq!(bad_add(1, 2), 3);
  }

  #[test]
  fn test_divide() {
    assert_eq!(divide_non_zero_result(10, 2), 5);
  }

  #[test]
  #[should_panic]
  fn test_any_panic() {
    divide_non_zero_result(1, 0);
  }

  #[test]
  #[should_panic(expected = "Divide result is zero")]
  fn test_specific_panic() {
    divide_non_zero_result(1, 10);
  }

  #[test]
  #[ignore]
  fn ignored_test() {
    assert_eq!(add(0, 0), 0);
  }
}
