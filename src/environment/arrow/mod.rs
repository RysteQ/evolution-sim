pub mod arrow_maker;
pub mod direction;
mod brain;

use crate::environment::arrow::arrow_maker::ArrowMaker;
use crate::environment::arrow::direction::Direction;
use crate::environment::arrow::brain::Brain;

pub struct Arrow {
  coords: Option<[u32; 2]>,
  brain: Brain,
  symbol: char,
  speed: f32,
  health: i32,
  eyesight: u32,
  previous_movement: Direction
}

impl From<&ArrowMaker> for Arrow {
  fn from(arrow_maker: &ArrowMaker) -> Self {
    Self::new(
      arrow_maker.get_brain(),
      arrow_maker.get_speed(),
      arrow_maker.get_health(),
      arrow_maker.get_eyesight()
    )
  }
}

impl Arrow {
  pub fn tick(&mut self) {  // returns it's health
    let decision = self.brain.make_decision();
    self.make_move(decision);
  }

  pub fn fight(mut arrows: Vec<Self>) {
    let mut total_damage = 0;
    arrows.iter().for_each(|arrow| total_damage += arrow.get_health());            // there must be a better way to 
    arrows.iter_mut().for_each(|arrow| arrow.take_damage(total_damage));           // chain this stuff together
  }

  fn new(brain: Brain, speed: f32, health: i32, eyesight: u32) -> Self {
    Self {
      coords: None,
      brain,
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

  pub fn get_health(&self) -> i32 {
    self.health
  }

  pub fn take_damage(&mut self, damage: i32) {
    self.health -= damage;
  }
  
  pub fn get_symbol(&self) -> char {
    self.symbol
  }

  fn make_move(&mut self, direction: Direction) {
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
