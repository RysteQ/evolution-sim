use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use crate::learning::neural_net::node::Node;
use crate::learning::neural_net::net_aspect::NetAspect;

#[derive(Clone, Debug, Default)]
pub struct Edge {
  output_node: Option<Arc<Mutex<Node>>>,
  weight: f32,
  through_value: Option<f32>
}

impl NetAspect for Edge {
  fn give_value(&mut self, value: f32) {
    self.through_value = Some(value);
  }
}

impl Edge {
  pub fn fire(&mut self, value: f32) {
    self.give_value(value);
    self.apply_weight();
    self.send_value();
  }

  pub fn give_node(&mut self, node: Node) {
    self.output_node = Some(Arc::new(Mutex::new(node)));
  }

  pub fn with_node(node: Node) -> Self {
    Self {
      output_node: Some(Arc::new(Mutex::new(node))),
      ..Default::default()
    }
  }
  
  fn apply_weight(&mut self) {
    self.give_value(self.weight * self.through_value.unwrap());
  }

  fn send_value(&mut self) {
    self.output_node.as_ref().unwrap().borrow_mut().lock().unwrap().fire(self.through_value.unwrap());
  }
}
