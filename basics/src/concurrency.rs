use std::thread;

pub fn value_move_into_closure() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  handle.join().unwrap();
}

pub mod message_passing {
  use std::sync::mpsc;
  use std::thread;
  // use std::time::Duration;

  pub fn send_single_message() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
      let val = String::from("hi");
      tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
  }

  pub fn send_multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
      ];

      for val in vals {
        tx.send(val).unwrap();
        // thread::sleep(Duration::from_secs(1));
      }
    });

    for received in rx {
      println!("Got: {}", received);
    }
  }

  pub fn send_messages_from_multiple_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
      ];

      for val in vals {
        tx1.send(val).unwrap();
        // thread::sleep(Duration::from_secs(1));
      }
    });

    thread::spawn(move || {
      let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
      ];

      for val in vals {
        tx.send(val).unwrap();
        // thread::sleep(Duration::from_secs(1));
      }
    });

    for received in rx {
      println!("Got: {}", received);
    }
  }
}
