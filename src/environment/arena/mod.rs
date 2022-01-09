mod powerup;
pub mod modifier;
mod distributor;


use crate::environment::arena::powerup::Powerup;
use crate::environment::arena::distributor::Distributor;
use crate::environment::arrow::Arrow;
use crate::environment::arrow::arrow_maker::ArrowMaker;


pub struct Arena {
  dimensions: [u32; 2],
  powerups: Vec<Powerup>,
  arrows: Vec<Arrow>,
  distributor: Distributor,
}

impl Arena {
  pub fn new(
    dimensions: [u32; 2], 
    powerup_amount: u32, 
    arrow_amount: u32, 
    arrow_maker: ArrowMaker
  ) -> Self {

    let mut distributor = Distributor::new(dimensions);
    let mut powerups = Vec::new();
    let mut arrows = Vec::new();
    for _ in 0..powerup_amount {
      powerups.push(distributor.make_powerup(None))
    }
    for _ in 0..arrow_amount {
      arrows.push(distributor.make_arrow(&arrow_maker));
    }
    
    
    Self {
      dimensions,
      powerups,
      arrows,
      distributor
    }
  }

  pub fn render(&self) {
    for y in 0..self.dimensions[1] {
      'x_loop: for x in 0..self.dimensions[0] {

        for arrow in &self.arrows {
          if arrow.get_coords() == [x, y] {
            print!("{}", arrow.get_symbol());
            continue 'x_loop;
          }
        }
        for powerup in &self.powerups {
          if powerup.get_coords() == &[x, y] {
            print!("{}", powerup.get_symbol());
            continue 'x_loop;
          }
        }
        
        
        if {
          x == 0 || x == self.dimensions[0] - 1 ||
          y == 0 || y == self.dimensions[1] - 1
        } {
          print!("█");
        } else {
          print!(" ");
        }
      }
      print!("\n");
    }
  }

  pub fn update(&mut self) {
    // let the arrows do their movement
    self.arrows.iter_mut().for_each(|arrow| arrow.tick());

    
    // wrap before attacking so they don't wrap into another
    self.wrap_around_edges();

    
    // remove the ones which are dead
    self.arrows = {
      let mut new_arrows = Vec::new();
      for _ in 0..self.arrows.len() {
        new_arrows.push(self.arrows.pop().unwrap())
      }
      new_arrows
    };
  }

  fn wrap_around_edges(&mut self) {
    for i in 0..self.arrows.len() {
      let arrow = self.arrows.get_mut(i).unwrap();
      let coords = arrow.get_coords();
      if coords[0] == 0 {
        arrow.set_coords([self.dimensions[0] - 2, coords[1]])
      } else if coords[0] == self.dimensions[0] - 1 {
        arrow.set_coords([1, coords[1]])
      }
      if coords[1] == 0 {
        arrow.set_coords([coords[0], self.dimensions[1] - 2])
      } else if coords[1] == self.dimensions[1] - 1 {
        arrow.set_coords([coords[0], 1])
      }
    }
  }
}
