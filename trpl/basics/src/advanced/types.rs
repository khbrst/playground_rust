#[allow(unused_variables)]
pub fn use_type_alias() {
  type Thunk = Box<dyn Fn() + Send + 'static>;

  let f: Thunk = Box::new(|| println!("hi"));
}

pub fn allow_dynamically_sized_type_with_generic() {
  #[allow(unused_variables)]
  fn generic<T: ?Sized>(t: &T) {}
  generic("test");
}
