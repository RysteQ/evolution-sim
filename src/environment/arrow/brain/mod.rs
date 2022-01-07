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
  pub fn default_with_neural_net(hidden_layers: u32) -> Self {
    Self {
      neural_net: NeuralNet::new(hidden_layers),
      bias: Bias::default(),
      vision: Vision::default(),
    }
  }
  
  pub fn make_decision(&mut self) -> Direction {
    Decisions::from(self.neural_net.get_decision_array({
      self.bias.get_inputs(self.vision.clone())
    })).make_decision()
  }
}
