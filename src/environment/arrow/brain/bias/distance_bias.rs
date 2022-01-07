/* 
  this is to turn the distance of something 
  into how much the arrow cares about it
*/

#[derive(Clone)]
pub struct DistanceBias(f32);

impl Default for DistanceBias {
  fn default() -> Self {
    Self(1.0)
  }
}

impl DistanceBias {
  pub fn get_bias(&self) -> f32 {
    self.0
  }

  pub fn change_bias_f32(&mut self, amount: f32) {
    self.0 += amount;
  }

  pub fn change_bias_array(&mut self, relative_coords: [u32; 2]) {
    self.change_bias_f32({ (
        ( relative_coords[0] * relative_coords[0] ) + 
        ( relative_coords[1] * relative_coords[1] )
    ) as f32 }.sqrt());
  }
}
