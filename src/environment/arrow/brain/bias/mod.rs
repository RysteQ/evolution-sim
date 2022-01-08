mod object_bias;
mod distance_bias;
mod object_type;

use crate::environment::arrow::brain::bias::distance_bias::DistanceBias;
use crate::environment::arrow::brain::bias::object_bias::ObjectBias;
use crate::environment::arrow::brain::vision::Vision;

#[derive(Default, Clone)]
pub struct Bias {
  distance_bias: DistanceBias,
  object_bias: ObjectBias,
}

impl Bias {
  pub fn get_inputs(&self, vision: &Vision) -> Vec<f32> {
    let arrows = vision.get_arrows();
    let powerups = vision.get_powerups();

    let mut ret_vec = Vec::new();

    for bias in self.object_bias.get_biases() {
      ret_vec.push(bias);
    }
    ret_vec.push(self.distance_bias.get_bias());

    
    ret_vec
  }
}
