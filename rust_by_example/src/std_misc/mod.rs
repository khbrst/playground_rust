mod child_processes;
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

pub fn run_child_processes() {
  child_processes::command();
  child_processes::pipes();
  child_processes::wait();
}

pub fn filesystem_operations() {
  use std::fs;
  use std::fs::{File, OpenOptions};
  use std::io;
  use std::io::prelude::*;
  use std::os::unix;
  use std::path::Path;

  // A simple implementation of `% cat path`
  fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
  }

  // A simple implementation of `% echo s > path`
  fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
  }

  // A simple implementation of `% touch path` (ignores existing files)
  fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
    }
  }

  println!("`mkdir a`");
  // Create a directory, returns `io::Result<()>`
  match fs::create_dir("a") {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(_) => {}
  }

  println!("`echo hello > a/b.txt`");
  // The previous match can be simplified using the `unwrap_or_else` method
  echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("`mkdir -p a/c/d`");
  // Recursively create a directory, returns `io::Result<()>`
  fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("`touch a/c/e.txt`");
  touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("`ln -s ../b.txt a/c/b.txt`");
  // Create a symbolic link, returns `io::Result<()>`
  if cfg!(target_family = "unix") {
    unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
      println!("! {:?}", why.kind());
    });
  }

  println!("`cat a/c/b.txt`");
  match cat(&Path::new("a/c/b.txt")) {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(s) => println!("> {}", s),
  }

  println!("`ls a`");
  // Read the contents of a directory, returns `io::Result<Vec<Path>>`
  match fs::read_dir("a") {
    Err(why) => println!("! {:?}", why.kind()),
    Ok(paths) => for path in paths {
      println!("> {:?}", path.unwrap().path());
    },
  }

  println!("`rm a/c/e.txt`");
  // Remove a file, returns `io::Result<()>`
  fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  println!("`rmdir a/c/d`");
  // Remove an empty directory, returns `io::Result<()>`
  fs::remove_dir("a/c/d").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
}

pub fn foreign_function_interface() {
  use std::fmt;

  // this extern block links to the libm library
  #[link(name = "m")]
  extern "C" {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
  }

  // Since calling foreign functions is considered unsafe,
  // it's common to write safe wrappers around them.
  fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
  }

  // Minimal implementation of single precision complex numbers
  #[repr(C)]
  #[derive(Clone, Copy)]
  struct Complex {
    re: f32,
    im: f32,
  }

  impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      if self.im < 0. {
        write!(f, "{}-{}i", self.re, -self.im)
      } else {
        write!(f, "{}+{}i", self.re, self.im)
      }
    }
  }

  // z = -1 + 0i
  let z = Complex { re: -1., im: 0. };

  // calling a foreign function is an unsafe operation
  let z_sqrt = unsafe { csqrtf(z) };

  println!("the square root of {:?} is {:?}", z, z_sqrt);

  // calling safe API wrapped around unsafe operation
  println!("cos({:?}) = {:?}", z, cos(z));
}
