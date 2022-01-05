mod environment;

use std::sync::{Arc, Mutex};
use std::{thread, time};
use crate::environment::arena::Arena;

macro_rules! sleep_clear_screen {
  ($wait_time:expr) => {
    thread::sleep($wait_time);
    print!("\x1B[2J\x1B[1;1H");
  }
}

fn main() {

  for _ in 0..10 {
    println!();
  }

  let arena_counter = Arc::new(Mutex::new(Arena::new([150, 25])));

  let render_arena_counter = Arc::clone(&arena_counter);
  thread::spawn(move || {
    let render_wait_time = time::Duration::from_millis(500);

    loop {
      sleep_clear_screen!(render_wait_time);
      let mut arena = render_arena_counter.lock().unwrap();

      arena.render();
    }
  });



  let update_wait_time = time::Duration::from_millis(600);
  loop {
    thread::sleep(update_wait_time);
    // arena_counter.lock().unwrap().foo();
  }
}
