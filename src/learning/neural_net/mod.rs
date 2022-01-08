use crate::learning::neural_net::matrix::NetMatrix;

type Layer = Vec<NetMatrix>;

#[derive(Clone)]
pub struct NeuralNet {
  node_layers: Layer,
  edge_layers: Layer,
}