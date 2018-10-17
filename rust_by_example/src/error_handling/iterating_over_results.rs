pub fn without_handling() {
  let strings = vec!["tofu", "93", "18"];
  let possible_numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
  println!("Results: {:?}", possible_numbers);
}

pub fn ignore_the_failed_items() {
  let strings = vec!["tofu", "93", "18"];
  let numbers: Vec<_> = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .filter_map(Result::ok)
    .collect();
  println!("Results: {:?}", numbers);
}

pub fn fail_the_entire_operation() {
  let strings = vec!["tofu", "93", "18"];
  let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
  println!("Results: {:?}", numbers);
}

pub fn collect_all_valid_values_and_failures() {
  let strings = vec!["tofu", "93", "18"];
  let (numbers, errors): (Vec<_>, Vec<_>) = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
  println!("Numbers: {:?}", numbers);
  println!("Errors: {:?}", errors);
}

pub fn collect_all_valid_values_and_failures_with_unwrap() {
  let strings = vec!["tofu", "93", "18"];
  let (numbers, errors): (Vec<_>, Vec<_>) = strings
    .into_iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
  let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
  let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
  println!("Numbers: {:?}", numbers);
  println!("Errors: {:?}", errors);
}
