use crate::environment::arena::powerup::*;
use crate::environment::arena::modifier::Modifier;
use rand::Rng;
use crate::environment::arrow::Arrow;
use crate::environment::arrow::arrow_maker::ArrowMaker;

#[macro_export]
macro_rules! gen_coords {
  ($dimensions:expr) => {{
    let mut rng = rand::thread_rng();
    [
      rng.gen_range(1..$dimensions[0] - 1),
      rng.gen_range(1..$dimensions[1] - 1)
    ]
  }}
}


pub struct Distributor {
  dimensions: [u32; 2],
  taken_coords: Vec<[u32; 2]>
}

impl Distributor {
  pub fn new(
    dimensions: [u32; 2]
  ) -> Self {
    Self {
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

    
    self.taken_coords.push(potential_coords);
    from_modifier(&potential_coords, modifier)
  }

  pub fn make_arrow(&mut self, arrow_maker: &ArrowMaker) -> Arrow {
    let mut potential_coords = gen_coords!(self.dimensions);
    {
      let mut i = 0;
      while self.taken_coords.contains(&potential_coords) &&
        i < self.dimensions[0] * self.dimensions[1] {
        potential_coords = gen_coords!(self.dimensions);
        i += 1;
      }
      if self.taken_coords.contains(&potential_coords) {
        panic!("too many arrows and not enough space!");
      }
    }
    
    self.taken_coords.push(potential_coords);
    let mut arrow = Arrow::from(arrow_maker);
    arrow.set_coords(potential_coords);
    arrow
  }
}