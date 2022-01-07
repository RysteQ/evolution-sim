mod edge;
mod node;
mod net_aspect;

use crate::learning::neural_net::node::Node;

#[derive(Clone, Default)]
pub struct NeuralNet {
  input_layer: Vec<Node>,
  hidden_layers: Vec<Vec<Node>>,
  output_layer: Vec<Node>,
}

impl NeuralNet {
  pub fn get_decision_array(&mut self, inputs: Vec<f32>) -> [f32; 5] {
    self.kick_off(inputs);
    let outputs = self.get_outputs();
    [
      outputs[0],
      outputs[1],
      outputs[2],
      outputs[3],
      outputs[4]
    ]
  }

  fn kick_off(&mut self, inputs: Vec<f32>) {
    for i in 0..self.input_layer.len() {
      self.input_layer.get_mut(i).unwrap().fire(*inputs.get(i).unwrap());
    }
  }

  fn get_outputs(&self) -> Vec<f32> {
    self.output_layer.iter().map(|node| node.get_through_value()).collect()
  }
}
