mod environment;

use crate::environment::arena::Arena;

fn main() {
  let arena = Arena::new([40, 20]);
  arena.render();
}
