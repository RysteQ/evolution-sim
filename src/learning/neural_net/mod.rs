mod edge;
mod node;
mod net_aspect;

use crate::learning::neural_net::node::Node;
use crate::learning::neural_net::edge::Edge;

#[derive(Clone)]
pub struct NeuralNet {
  input_layer: Vec<Node>,
  hidden_layers: Vec<Vec<Node>>,
  output_layer: Vec<Node>,
}

impl NeuralNet {
  pub fn new(hidden_layers_count: u32) -> Self {
    // make the nodes for each layer
    
    let mut output_layer = Vec::new();
    for _ in 0..5 {  // number of output variables
      output_layer.push(Node::default());
    }

    let mut hidden_layers = Vec::new();
    for _ in 0..hidden_layers_count {
      hidden_layers.push({
        let mut layer = Vec::new();
        for _ in 0..10 {  // number of hidden nodes per layer
          layer.push(Node::default());
        }
        layer
      });
    }
    
    let mut input_layer = Vec::new();
    for _ in 0..5 {  // number of input parameters
      input_layer.push(Node::default());
    }


    // make and attach the edges for each layer
    {
      let next_layer_size = hidden_layers.get(0).unwrap().len();
      for node in input_layer.iter_mut() {
        let mut edges_vec = Vec::new();
        for _ in 0..next_layer_size {
          edges_vec.push(Edge::default());
        }
        node.give_edges(edges_vec);
      }
    }

    
    
    Self {
      input_layer,
      hidden_layers,
      output_layer,
    }
  }
  
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
    //println!("{:#?}", self.output_layer);
    self.output_layer.iter().map(|node| node.get_through_value()).collect()
  }
}
