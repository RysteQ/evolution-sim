/* 
  this is to turn the type of thing on the board
  into a multiplier for how much an arrow cares
  about an arrow or a certain type modifier of powerup
*/

use crate::environment::arrow::brain::bias::object_type::ObjectType;

#[derive(Clone)]
pub struct ObjectBias {
  arrow: f32,
  eyesight: f32,
  speed: f32,
  health: f32,
}

impl Default for ObjectBias {
  fn default() -> Self {
    Self {
      arrow: 1.0,
      eyesight: 1.0,
      speed: 1.0,
      health: 1.0,
    }
  }
}

impl ObjectBias {
  pub fn get_biases(&self) -> [f32; 4] {
    [
      self.arrow,
      self.eyesight,
      self.speed,
      self.health
    ]
  }

  pub fn change_bias(&mut self, object_type: ObjectType, amount: f32) {
    match object_type {
      ObjectType::PowerupObject(powerup_type) => match powerup_type {
        Modifier::Eyesight => self.eyesight += amount,
        Modifier::Speed => self.speed += amount,
        Modifier::Health => self.health += amount,
      }
      ObjectType::ArrowObject => self.arrow += amount,
    }
  }
}
