mod arrow_maker;
mod direction;

use crate::environment::arrow::arrow_maker::ArrowMaker;
use crate::environment::arrow::direction::Direction;
use rand::Rng;
use crate::gen_coords;

pub struct Arrow {
  coords: Option<[u32; 2]>,
  symbol: char,
  speed: f32,
  health: u32,
  eyesight: u32,
  previous_movement: Option<Direction>
}

impl From<ArrowMaker> for Arrow {
  fn from(arrow_maker: ArrowMaker) -> Self {
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
      previous_movement: None
    }
  }

  pub fn make_move(&mut self, direction: Option<Direction>) {
    match direction {
      Some(direction) => {
        match direction {
          Direction::Up => {
            self.coords.unwrap()[1] = { self.coords.unwrap()[1] as f32 - self.speed } as u32;
            self.symbol = '^';
          }
          Direction::Left => {
            self.coords.unwrap()[0] = { self.coords.unwrap()[1] as f32 - self.speed } as u32;
            self.symbol = '<';
          }
          Direction::Down => {
            self.coords.unwrap()[1] = { self.coords.unwrap()[1] as f32 + self.speed } as u32;
            self.symbol = 'v';
          }
          Direction::Right => {
            self.coords.unwrap()[0] = { self.coords.unwrap()[1] as f32 + self.speed } as u32;
            self.symbol = '>';
          }
        }
      }
      None => self.symbol = '•',
    }
  }
}
