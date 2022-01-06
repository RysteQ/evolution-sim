use crate::environment::arrow::brain::Brain;

pub struct ArrowMaker {
  brain: Brain,
  speed: f32,
  health: u32,
  eyesight: u32,
}

impl Default for ArrowMaker {
  fn default() -> Self {
    Self {
      brain: Brain::default(),
      speed: 1.0,
      health: 1,
      eyesight: 3
    }
  }
}

impl ArrowMaker {
  pub fn get_brain(&self) -> Brain {
    self.brain.clone()
  }
  
  pub fn get_speed(&self) -> f32 {
    self.speed
  }

  pub fn get_health(&self) -> u32 {
    self.health
  }

  pub fn get_eyesight(&self) -> u32 {
    self.eyesight
  }
}
