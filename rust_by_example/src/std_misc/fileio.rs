pub fn do_open() {
  use std::error::Error;
  use std::fs::File;
  use std::io::prelude::*;
  use std::path::Path;

  // Create a path to the desired file
  let path = Path::new("static").join("hello.txt");
  let display = path.display();

  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
    // The `description` method of `io::Error` returns a string that
    // describes the error
    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    Ok(_) => print!("{} contains:\n{}", display, s),
  }

  // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn do_create() {
  static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

  use std::error::Error;
  use std::fs::File;
  use std::io::prelude::*;
  use std::path::Path;

  let path = Path::new("out/lorem_ipsum.txt");
  let display = path.display();

  // Open a file in write-only mode, returns `io::Result<File>`
  let mut file = match File::create(&path) {
    Err(why) => panic!("couldn't create {}: {}", display, why.description()),
    Ok(file) => file,
  };

  // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
  match file.write_all(LOREM_IPSUM.as_bytes()) {
    Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
    Ok(_) => println!("successfully wrote to {}", display),
  }
}
