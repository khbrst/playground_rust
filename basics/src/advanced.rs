#[allow(unused_variables)]
pub fn create_raw_pointer() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  let address = 0x012345usize;
  let r = address as *const i32;
}

pub fn dereference_raw_pointer() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }
}

unsafe fn dangerous() {}

pub fn call_the_dangerous_function() {
  unsafe {
    dangerous();
  }
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
    )
  }
}

pub fn create_safe_abstract_on_unsafe_code() {
  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (a, b) = split_at_mut(r, 3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);
}

pub fn use_the_ffi() {
  extern "C" {
    fn abs(input: i32) -> i32;
  }

  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
  unsafe {
    COUNTER += inc;
  }
}

pub fn access_to_static_mutable_variable() {
  add_to_count(3);

  unsafe {
    println!("COUNTER: {}", COUNTER);
  }
}
