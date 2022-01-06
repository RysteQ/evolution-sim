use crate::environment::arena::modifier::Modifier;

#[derive(Clone)]
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
