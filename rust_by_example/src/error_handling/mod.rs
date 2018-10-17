mod iterating_over_results;
mod multiple_error_types;
mod option_and_unwrap;
mod result;

fn give_princess(gift: &str) {
  // Princesses hate snakes, so we need to stop if she disapproves!
  if gift == "snake" {
    // panic!("AAAaaaaa!!!!");
    println!("AAAaaaaa!!!!");
    return;
  }

  println!("I love {}s!!!!!", gift);
}

pub fn panic() {
  give_princess("teddy bear");
  give_princess("snake");
}

pub fn run_option_and_unwrap() {
  option_and_unwrap::basic();
  option_and_unwrap::combinators_map();
  option_and_unwrap::combinators_and_then();
}

pub fn run_result() {
  result::use_unwrap();
  result::use_match_before_use_combinators();
  result::use_combinators();
  result::aliases();
  result::early_returns();
  result::introducing_question_mark();
  result::try_macro();
}

pub fn run_multiple_error_types() {
  multiple_error_types::before_handling();
  multiple_error_types::pulling_results_out_of_options();
  multiple_error_types::pulling_results_out_of_options_with_combinators();
  multiple_error_types::defining_an_error_type();
  multiple_error_types::boxing_errors();
  multiple_error_types::other_uses_of_question_mark();
  multiple_error_types::wrapping_errors();
}

pub fn run_iterating_over_results() {
  iterating_over_results::without_handling();
  iterating_over_results::ignore_the_failed_items();
  iterating_over_results::fail_the_entire_operation();
  iterating_over_results::collect_all_valid_values_and_failures();
  iterating_over_results::collect_all_valid_values_and_failures_with_unwrap();
}
