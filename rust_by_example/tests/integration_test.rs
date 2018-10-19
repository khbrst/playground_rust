// extern crate we're testing, same as any other code would do.
extern crate rust_by_example;

mod common;

#[test]
fn test_add() {
  common::setup();
  assert_eq!(rust_by_example::testing::add(3, 2), 5);
}
