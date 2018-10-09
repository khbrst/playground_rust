pub fn if_else() {
  let n = 5;

  if n < 0 {
    print!("{} is negative", n);
  } else if n > 0 {
    print!("{} is positive", n);
  } else {
    print!("{} is zero", n);
  }

  let big_n = if n < 10 && n > -10 {
    println!(", and is a small number, increase ten-fold");

    // This expression returns an `i32`.
    10 * n
  } else {
    println!(", and is a big number, half the number");

    // This expression must return an `i32` as well.
    n / 2
    // TODO ^ Try suppressing this expression with a semicolon.
  };
  //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

  println!("{} -> {}", n, big_n);
}

pub fn do_loop() {
  let mut count = 0u32;

  println!("Let's count until infinity!");

  // Infinite loop
  loop {
    count += 1;

    if count == 3 {
      println!("three");

      // Skip the rest of this iteration
      continue;
    }

    println!("{}", count);

    if count == 5 {
      println!("OK, that's enough");

      // Exit this loop
      break;
    }
  }
}
