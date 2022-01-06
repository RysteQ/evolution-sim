pub struct ArrowMaker {
  speed: f32,
  health: u32,
  eyesight: u32,
}

impl Default for ArrowMaker {
  fn default() -> Self {
    Self {
      speed: 1.0,
      health: 1,
      eyesight: 3
    }
  }
}

impl ArrowMaker {
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
