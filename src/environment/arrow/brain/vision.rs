use crate::environment::arena::modifier::Modifier;

#[derive(Clone, Debug)]
pub struct Vision {
  arrows: Vec<[u32; 2]>,
  powerups: Vec<(Modifier, [u32; 2])>,
}

impl Default for Vision {
  fn default() -> Self {
    Self {
      arrows: Vec::new(),
      powerups: Vec::new(),
    }
  }
}

impl Vision {
  pub fn get_arrows(&self) -> Vec<[u32; 2]> {
    self.arrows.clone()
  }

  pub fn get_powerups(&self) -> Vec<(Modifier, [u32; 2])> {
    self.powerups.clone()
  }
}
