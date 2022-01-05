use crate::environment::powerup::*;
use crate::environment::modifier::Modifier;
use rand::Rng;


macro_rules! gen_coords {
  ($dimensions:expr) => {{
    let mut rng = rand::thread_rng();
    [
      rng.gen_range(1..$dimensions[0] - 1),
      rng.gen_range(1..$dimensions[1] - 1)
    ]
  }}
}


pub struct PowerupDistribution {
  total_num: u32,
  concentration: f32,  // todo make this do something
  dimensions: [u32; 2],
  taken_coords: Vec<[u32; 2]>
}

impl PowerupDistribution {
  pub fn new(
    concentration: f32,
    dimensions: [u32; 2]
  ) -> Self {
    assert!(concentration > 0.0 && concentration <= 1.0);
    Self {
      total_num: 0,
      concentration,
      dimensions,
      taken_coords: vec![]
    }
  }

  pub fn make_powerup(
    &mut self,
    modifier: Option<Modifier>,
  ) -> Powerup {
    let modifier = match modifier {
      Some(modifier) => modifier,
      None => rand::random(),
    };

    let mut potential_coords = gen_coords!(self.dimensions);
    {
      let mut i = 0;
      while self.taken_coords.contains(&potential_coords) &&
        i < self.dimensions[0] * self.dimensions[1] {
        potential_coords = gen_coords!(self.dimensions);
        i += 1;
      }
      if self.taken_coords.contains(&potential_coords) {
        panic!("too many powerups and not enough space!");
      }
    }

    
    self.total_num += 1;
    self.taken_coords.push(potential_coords);
    from_modifier(&potential_coords, modifier)
  }
}