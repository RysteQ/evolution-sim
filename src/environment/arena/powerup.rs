use crate::environment::arena::modifier::Modifier;

#[derive(Debug)]
pub struct Powerup {
  coords: [u32; 2],
  modifier: Modifier,
  symbol: char,
}

impl Powerup {
  fn new(
    coords: [u32; 2],
    modifier: Modifier,
    symbol: char
  ) -> Self {
    Self {
      coords,
      modifier,
      symbol,
    }
  }

  pub fn get_type(&self) -> &Modifier {
    &self.modifier
  }

  pub fn get_symbol(&self) -> &char {
    &self.symbol
  }

  pub fn get_coords(&self) -> &[u32; 2] {
    &self.coords
  }
}

pub fn new_random_modifier(coords: [u32; 2]) -> Powerup {
  match rand::random::<Modifier>() {
    Modifier::Eyesight => new_eyesight(coords),
    Modifier::Speed => new_speed(coords),
    Modifier::Health => new_health(coords),
  }
}

pub fn new_speed(coords: [u32; 2]) -> Powerup {
  Powerup::new(coords, Modifier::Speed, 's')
}

pub fn new_eyesight(coords: [u32; 2]) -> Powerup {
  Powerup::new(coords, Modifier::Eyesight, 'e')
}

pub fn new_health(coords: [u32; 2]) -> Powerup {
  Powerup::new(coords, Modifier::Health, 'h')
}

pub fn from_modifier(coords: &[u32; 2], modifier: Modifier) -> Powerup {
  match modifier {
    Modifier::Eyesight => new_eyesight(*coords),
    Modifier::Speed => new_speed(*coords),
    Modifier::Health => new_health(*coords),
  }
}
