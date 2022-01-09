mod powerup;
pub mod modifier;
mod distributor;


use multimap::MultiMap;
use crate::environment::arena::powerup::Powerup;
use crate::environment::arena::distributor::Distributor;
use crate::environment::arrow::Arrow;
use crate::environment::arrow::arrow_maker::ArrowMaker;


#[derive(Debug)]
pub struct Arena {
  dimensions: [u32; 2],
  powerups: Vec<Powerup>,
  arrows: Vec<Arrow>,
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

    
    // deal with attacking
    let mut board_objects_multimap = MultiMap::new();
    self.arrows.iter().for_each(|arrow| board_objects_multimap.insert(arrow.get_coords().clone(), arrow.clone()));


    self.arrows = Vec::new();
    // wish I knew a more functional way to pull this off, shame I need to use for loops
    for (_, arrows_vec) in board_objects_multimap.iter_all() {
      Self::fight(arrows_vec).into_iter().for_each(|arrow| self.arrows.push(arrow));
    }

    
    // remove the ones which are dead last
    self.arrows = {
      let mut new_arrows = Vec::new();
      for _ in 0..self.arrows.len() {
        let potential_arrow = self.arrows.pop().unwrap();
        if potential_arrow.get_health() > 0 {
          new_arrows.push(potential_arrow);
        }
      }
      new_arrows
    };
  }

  pub fn fight(arrows: &Vec<Arrow>) -> Vec<Arrow> {
    let mut arrows = arrows.clone();
    let mut total_damage = 0;
    arrows.iter().for_each(|arrow| total_damage += arrow.get_health());                        // there must be a better way 
    arrows.iter_mut().for_each(|arrow| arrow.take_damage(total_damage - arrow.get_health()));  // to chain this stuff together
    arrows
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
