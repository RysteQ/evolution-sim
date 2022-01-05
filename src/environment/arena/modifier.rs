use rand::{
  distributions::{Distribution, Standard},
  Rng,
};

pub enum Modifier {
  Eyesight,
  Speed,
  Health,
}

impl Distribution<Modifier> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Modifier {
    match rng.gen_range(0..3) {
      0 => Modifier::Eyesight,
      1 => Modifier::Speed,
      _ => Modifier::Health,
    }
  }
}
