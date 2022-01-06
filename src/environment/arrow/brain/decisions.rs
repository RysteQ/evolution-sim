use crate::environment::arrow::direction::Direction;

#[derive(Clone)]
pub struct Decisions {
  up: f32,
  left: f32,
  down: f32,
  right: f32,
  still: f32,
}

impl Default for Decisions {
  fn default() -> Self {
    Self {
      up: 1.0,
      left: 1.0,
      down: 1.0,
      right: 1.0,
      still: 1.0,
    }
  }
}

impl Decisions {
  pub fn make_decision(&self) -> Direction {
    match {
      use ordered_float::NotNan;
      use rand::Rng;
      
      let mut rng = rand::thread_rng();
      let choices_vec = [
        rng.gen_range(0.0..self.up),
        rng.gen_range(0.0..self.left),
        rng.gen_range(0.0..self.down),
        rng.gen_range(0.0..self.right),
        rng.gen_range(0.0..self.still),
      ].iter().cloned()
        .map(NotNan::new)
        .filter_map(Result::ok)
        .collect::<Vec<NotNan<f32>>>();
      
      let max = choices_vec.iter().max().unwrap();
      
      choices_vec.iter().position(|x| x == max).unwrap()
    } {
      0 => Direction::Up,
      1 => Direction::Left,
      2 => Direction::Down,
      3 => Direction::Right,
      4 => Direction::NoDirection,
      _ => panic!("issue with getting a decisions from an arrow."),
    }
  }
}
