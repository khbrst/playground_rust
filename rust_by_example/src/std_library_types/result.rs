pub fn basic() {
  mod checked {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
      DivisionByZero,
      NonPositiveLogarithm,
      NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
      if y == 0.0 {
        // This operation would `fail`, instead let's return the reason of
        // the failure wrapped in `Err`
        Err(MathError::DivisionByZero)
      } else {
        // This operation is valid, return the result wrapped in `Ok`
        Ok(x / y)
      }
    }

    pub fn sqrt(x: f64) -> MathResult {
      if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
      } else {
        Ok(x.sqrt())
      }
    }

    pub fn ln(x: f64) -> MathResult {
      if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
      } else {
        Ok(x.ln())
      }
    }
  }

  // `op(x, y)` === `sqrt(ln(x / y))`
  fn op(x: f64, y: f64) -> f64 {
    // This is a three level match pyramid!
    match checked::div(x, y) {
      Err(why) => panic!("{:?}", why),
      Ok(ratio) => match checked::ln(ratio) {
        Err(why) => panic!("{:?}", why),
        Ok(ln) => match checked::sqrt(ln) {
          Err(why) => panic!("{:?}", why),
          Ok(sqrt) => sqrt,
        },
      },
    }
  }

  // Will this fail? => Ok, it will panic!
  // println!("{}", op(1.0, 10.0));
  println!("{}", op(1.0, 1.0));
}

pub fn question_mark() {
  mod checked {
    #[derive(Debug)]
    enum MathError {
      DivisionByZero,
      NonPositiveLogarithm,
      NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
      if y == 0.0 {
        Err(MathError::DivisionByZero)
      } else {
        Ok(x / y)
      }
    }

    fn sqrt(x: f64) -> MathResult {
      if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
      } else {
        Ok(x.sqrt())
      }
    }

    fn ln(x: f64) -> MathResult {
      if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
      } else {
        Ok(x.ln())
      }
    }

    // Intermediate function
    fn op_(x: f64, y: f64) -> MathResult {
      // if `div` "fails", then `DivisionByZero` will be `return`ed
      let ratio = div(x, y)?;

      // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
      let ln = ln(ratio)?;

      sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
      match op_(x, y) {
        Err(why) => panic!(match why {
          MathError::NonPositiveLogarithm => "logarithm of non-positive number",
          MathError::DivisionByZero => "division by zero",
          MathError::NegativeSquareRoot => "square root of negative number",
        }),
        Ok(value) => println!("{}", value),
      }
    }
  }

  // checked::op(1.0, 10.0);
  checked::op(1.0, 1.0);
}
