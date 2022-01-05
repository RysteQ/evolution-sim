use crate::environment::modifier::Modifier;

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
    Modifier::Strength => new_strength(coords),
    Modifier::Speed => new_speed(coords),
    Modifier::Health => new_health(coords),
  }
}

pub fn new_speed(coords: [u32; 2]) -> Powerup {
  Powerup::new(coords, Modifier::Speed, 's')
}

pub fn new_strength(coords: [u32; 2]) -> Powerup {
  Powerup::new(coords, Modifier::Strength, 'a')
}

pub fn new_health(coords: [u32; 2]) -> Powerup {
  Powerup::new(coords, Modifier::Health, 'h')
}

pub fn from_modifier(coords: &[u32; 2], modifier: Modifier) -> Powerup {
  match modifier {
    Modifier::Strength => new_strength(*coords),
    Modifier::Speed => new_speed(*coords),
    Modifier::Health => new_health(*coords),
  }
}
