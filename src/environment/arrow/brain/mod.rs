mod vision;
mod decisions;
mod bias;

use crate::environment::arrow::brain::vision::Vision;
use crate::environment::arrow::brain::decisions::Decisions;
use crate::environment::arrow::direction::Direction;
use crate::environment::arrow::brain::bias::Bias;
use crate::learning::neural_net::NeuralNet;

#[derive(Default, Clone)]
pub struct Brain {
  neural_net: NeuralNet,
  bias: Bias,
  viewing: Vision,
}

impl Brain {
  pub fn make_decision(&mut self) -> Direction {
    Decisions::from(self.neural_net.get_decision_array({
      
    })).make_decision()
  }
}
