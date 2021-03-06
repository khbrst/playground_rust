mod borrowing;
mod lifetimes;
mod ownership_and_moves;
mod raii;

pub fn run_raii() {
  raii::run();
  raii::destructor();
}

pub fn run_ownership_and_moves() {
  ownership_and_moves::basic();
  ownership_and_moves::mutability();
}

pub fn run_borrowing() {
  borrowing::basic();
  borrowing::mutability();
  borrowing::freezing();
  borrowing::aliasing();
  borrowing::the_ref_pattern();
}

pub fn run_lifetimes() {
  lifetimes::basic();
  lifetimes::explicit_annotation();
  lifetimes::functions();
  lifetimes::methods();
  lifetimes::structs();
  lifetimes::bounds();
  lifetimes::coercion();
  lifetimes::statics();
  lifetimes::elision();
}
