mod environment;

use std::sync::{Arc, Mutex};
use std::{thread, time};
use crate::environment::arena::Arena;
use crate::environment::arrow::arrow_maker::ArrowMaker;

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

  
  let arrow_maker = ArrowMaker::default();
  let arena_counter = Arc::new(Mutex::new(Arena::new([50, 25], 6, 10, arrow_maker)));

  let render_arena_counter = Arc::clone(&arena_counter);
  thread::spawn(move || {
    let render_wait_time = time::Duration::from_millis(500);

    loop {
      sleep_clear_screen!(render_wait_time);
      let arena = render_arena_counter.lock().unwrap();

      arena.render();
    }
  });

  thread::sleep(time::Duration::from_millis(250));

  let update_wait_time = time::Duration::from_millis(500);
  loop {
    thread::sleep(update_wait_time);
    let mut arena = arena_counter.lock().unwrap();
    arena.update();
  }
}
