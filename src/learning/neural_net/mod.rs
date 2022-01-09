mod layer;

use crate::learning::neural_net::layer::Layer;

#[derive(Clone, Debug)]
pub struct NeuralNet {
  node_layers: Vec<Layer>,
  edge_layers: Vec<Layer>,
}

impl NeuralNet {
  pub fn new(
    input_layer_size: u32, 
    hidden_layers_count: u32,
    hidden_layer_size: u32,
    output_layer_size: u32
  ) -> Self {
    // mega casting epic moment
    let (
      input_layer_size, 
      hidden_layers_count,
      hidden_layer_size,
      output_layer_size
    ) = (
      input_layer_size as usize, 
      hidden_layers_count as usize,
      hidden_layer_size as usize,
      output_layer_size as usize
    );
    
    Self {
      node_layers: {
        let mut node_layers = Vec::with_capacity(hidden_layers_count + 2);  // add two to account for the extra layers
        node_layers.push(Layer::with_size(hidden_layer_size, None));
        for _ in 0..hidden_layers_count {
          node_layers.push(Layer::with_size(hidden_layer_size, None))
        }
        node_layers.push(Layer::with_size(output_layer_size, None));
        node_layers
      },
      edge_layers: {
        let mut edge_layers = Vec::with_capacity(hidden_layers_count + 2);  // same here
        edge_layers.push(Layer::with_size(hidden_layer_size, Some(1.0)));
        for _ in 0..hidden_layers_count {
          edge_layers.push(Layer::with_size(hidden_layer_size, Some(1.0)))
        }
        edge_layers.push(Layer::with_size(output_layer_size, Some(1.0)));
        edge_layers
      }
    }
  }

  pub fn give_inputs(&mut self, inputs: Vec<f32>) {
    let mut new_inputs = Vec::with_capacity(inputs.len());

    for input in inputs {
      new_inputs.push(Some(input));
    }
    
    self.node_layers.get_mut(0).unwrap().set_value(new_inputs);
  }

  pub fn fire(&mut self) -> Vec<f32> {
    self.learn();
    
    let mut ret_vec: Vec<f32> = Vec::new();
    {
      let option_vec = self.node_layers.last().unwrap().get_values();
      for value in option_vec {
        ret_vec.push(value.unwrap())
      }
    }
    ret_vec
  }

  fn learn(&mut self) {
    for i in 0..(self.node_layers.len() - 1) {
      let mut next_layer = self.node_layers.get(i).unwrap().clone();
      next_layer = next_layer.multiply(self.edge_layers.get(i).unwrap());
      *self.node_layers.get_mut(i + 1).unwrap() = next_layer;
    }
  }
}
