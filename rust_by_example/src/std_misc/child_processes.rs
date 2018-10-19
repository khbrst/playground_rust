pub fn command() {
  use std::process::Command;

  let output = Command::new("rustc")
    .arg("--version")
    .output()
    .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

  if output.status.success() {
    let s = String::from_utf8_lossy(&output.stdout);

    print!("rustc succeeded and stdout was:\n{}", s);
  } else {
    let s = String::from_utf8_lossy(&output.stderr);

    print!("rustc failed and stderr was:\n{}", s);
  }
}

pub fn pipes() {
  use std::error::Error;
  use std::io::prelude::*;
  use std::process::{Command, Stdio};

  static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

  // Spawn the `wc` command
  let process = match Command::new("wc")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
  {
    Err(why) => panic!("couldn't spawn wc: {}", why.description()),
    Ok(process) => process,
  };

  // Write a string to the `stdin` of `wc`.
  //
  // `stdin` has type `Option<ChildStdin>`, but since we know this instance
  // must have one, we can directly `unwrap` it.
  match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
    Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
    Ok(_) => println!("sent pangram to wc"),
  }

  // Because `stdin` does not live after the above calls, it is `drop`ed,
  // and the pipe is closed.
  //
  // This is very important, otherwise `wc` wouldn't start processing the
  // input we just sent.

  // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
  let mut s = String::new();
  match process.stdout.unwrap().read_to_string(&mut s) {
    Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
    Ok(_) => print!("wc responded with:\n{}", s),
  }
}

pub fn wait() {
  use std::process::Command;

  let mut child = Command::new("sleep").arg("0").spawn().unwrap();
  let _result = child.wait().unwrap();

  println!("reached end of main");
}
