mod vision;
mod decisions;
mod bias;

use crate::environment::arrow::brain::vision::Vision;
use crate::environment::arrow::brain::decisions::Decisions;
use crate::environment::arrow::direction::Direction;
use crate::environment::arrow::brain::bias::Bias;
use crate::learning::neural_net::NeuralNet;

#[derive(Clone)]
pub struct Brain {
  neural_net: NeuralNet,
  bias: Bias,
  vision: Vision,
}

impl Brain {
  pub fn default_with_neural_net(
    input_layer_size: u32, 
    hidden_layers_count: u32,
    hidden_layer_size: u32,
    output_layer_size: u32,
  ) -> Self {
    Self {
      neural_net: NeuralNet::new(
        input_layer_size,
        hidden_layers_count,
        hidden_layer_size,
        output_layer_size
      ),
      bias: Bias::default(),
      vision: Vision::default(),
    }
  }
  
  pub fn make_decision(&mut self) -> Direction {
    self.neural_net.give_inputs(self.bias.get_inputs(&self.vision));
    Decisions::from(self.neural_net.fire().as_slice()).make_decision()
  }
}
