use std::cell::RefCell;
use std::rc::Rc;
use crate::learning::neural_net::node::Node;
use crate::learning::neural_net::net_aspect::NetAspect;

pub struct Edge {
  output_node: Rc<RefCell<Node>>,
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
  
  fn apply_weight(&mut self) {
    self.through_value = Some(self.weight * self.through_value.unwrap());
  }

  fn send_value(&self) {
    
  }
}
