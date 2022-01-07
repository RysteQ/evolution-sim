mod edge;
mod node;
mod net_aspect;

use crate::learning::neural_net::edge::Edge;
use crate::learning::neural_net::node::Node;

pub struct NeuralNet {
  hidden_layer_count: u32,
  nodes: Vec<Node>,
  edges: Vec<Edge>,
}
