pub enum Direction {
  Up,
  Left,
  Down,
  Right,
}

impl Direction {
  pub fn reverse(&self) -> Self {
    match self {
      Self::Up => Self::Down,
      Self::Down => Self::Up,
      Self::Right => Self::Left,
      Self::Left => Self::Right,
    }
  }
}
