use crate::environment::powerup::*;
use crate::environment::powerup_distribution::PowerupDistribution;


pub struct Arena {
  dimensions: [u32; 2],
  powerups: Vec<Powerup>
}

impl Arena {
  pub fn new(dimensions: [u32; 2]) -> Self {

    let mut powerup_distributor = PowerupDistribution::new(1.0, dimensions);
    let mut powerups = Vec::new();
    for _ in 0..4 {
      powerups.push(powerup_distributor.make_powerup(None))
    }
    
    
    Self {
      dimensions,
      powerups
    }
  }

  pub fn render(self) {
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
}
