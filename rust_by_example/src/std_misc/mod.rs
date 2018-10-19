mod fileio;
mod threads;

pub fn run_threads() {
  threads::basic();
  threads::map_reduce();
  threads::improved_map_reduce();
}

pub fn channels() {
  use std::sync::mpsc;
  use std::sync::mpsc::{Receiver, Sender};
  use std::thread;

  static NTHREADS: i32 = 3;

  // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
  // where `T` is the type of the message to be transferred
  // (type annotation is superfluous)
  let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

  for id in 0..NTHREADS {
    // The sender endpoint can be copied
    let thread_tx = tx.clone();

    // Each thread will send its id via the channel
    thread::spawn(move || {
      // The thread takes ownership over `thread_tx`
      // Each thread queues a message in the channel
      thread_tx.send(id).unwrap();

      // Sending is a non-blocking operation, the thread will continue
      // immediately after sending its message
      println!("thread {} finished", id);
    });
  }

  // Here, all the messages are collected
  let mut ids = Vec::with_capacity(NTHREADS as usize);
  for _ in 0..NTHREADS {
    // The `recv` method picks a message from the channel
    // `recv` will block the current thread if there are no messages available
    ids.push(rx.recv());
  }

  // Show the order in which the messages were sent
  println!("{:?}", ids);
}

pub fn path() {
  use std::path::Path;

  // Create a `Path` from an `&'static str`
  let path = Path::new(".");

  // The `display` method returns a `Show`able structure
  let _display = path.display();

  // `join` merges a path with a byte container using the OS specific
  // separator, and returns the new path
  let new_path = path.join("a").join("b");

  // Convert the path into a string slice
  match new_path.to_str() {
    None => panic!("new path is not a valid UTF-8 sequence"),
    Some(s) => println!("new path is {}", s),
  }
}

pub fn run_fileio() {
  fileio::do_open();
  fileio::do_create();
}
