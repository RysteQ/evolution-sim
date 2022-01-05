mod powerup;
mod modifier;
mod distributor;


use crate::environment::arena::powerup::Powerup;
use crate::environment::arena::distributor::Distributor;
use crate::environment::arrow::Arrow;


pub struct Arena {
  dimensions: [u32; 2],
  powerups: Vec<Powerup>,
  distributor: Distributor,
  arrows: Vec<Arrow>
}

impl Arena {
  pub fn new(dimensions: [u32; 2], arrows: u32) -> Self {

    let mut distributor = Distributor::new(dimensions);
    let mut powerups = Vec::new();
    for _ in 0..4 {
      powerups.push(distributor.make_powerup(None))
    }
    for _ in 0..arrows {
      arrows.push(distributor)
    }
    
    
    Self {
      dimensions,
      powerups,
      distributor
    }
  }

  pub fn render(&self) {
    for y in 0..self.dimensions[1] {
      'x_loop: for x in 0..self.dimensions[0] {
        
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

  pub fn foo(&mut self) {
    self.powerups.push(self.distributor.make_powerup(None));
  }
}
