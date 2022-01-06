pub mod arrow_maker;
pub mod direction;

use crate::environment::arrow::arrow_maker::ArrowMaker;
use crate::environment::arrow::direction::Direction;

pub struct Arrow {
  coords: Option<[u32; 2]>,
  symbol: char,
  speed: f32,
  health: u32,
  eyesight: u32,
  previous_movement: Direction
}

impl From<&ArrowMaker> for Arrow {
  fn from(arrow_maker: &ArrowMaker) -> Self {
    Self::new(
      arrow_maker.get_speed(),
      arrow_maker.get_health(),
      arrow_maker.get_eyesight()
    )
  }
}

impl Arrow {
  fn new(speed: f32, health: u32, eyesight: u32) -> Self {
    Self {
      coords: None,
      symbol: '•',
      speed,
      health,
      eyesight,
      previous_movement: Direction::NoDirection
    }
  }

  pub fn set_coords(&mut self, coords: [u32; 2]) {
    self.coords = Some(coords);
  }

  pub fn get_coords(&self) -> [u32; 2] {
    self.coords.unwrap()
  }

  pub fn get_symbol(&self) -> char {
    self.symbol
  }

  pub fn make_move(&mut self, direction: Direction) {
    match direction {
        Direction::Up => {
          let mut new_coords = self.coords.take().unwrap();
          new_coords[1] -= self.speed as u32;
          self.coords = Some(new_coords);
          self.symbol = '^';
        }
        Direction::Left => {
          let mut new_coords = self.coords.take().unwrap();
          new_coords[0] -= self.speed as u32;
          self.coords = Some(new_coords);
          self.symbol = '<';
        }
        Direction::Down => {
          let mut new_coords = self.coords.take().unwrap();
          new_coords[1] += self.speed as u32;
          self.coords = Some(new_coords);
          self.symbol = 'v';
        }
        Direction::Right => {
          let mut new_coords = self.coords.take().unwrap();
          new_coords[0] += self.speed as u32;
          self.coords = Some(new_coords);
          self.symbol = '>';
        }
        Direction::NoDirection => self.symbol = '•',
      }
    self.previous_movement = direction;
  }
}
