use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use crate::learning::neural_net::edge::Edge;
use crate::learning::neural_net::net_aspect::NetAspect;

#[derive(Clone)]
pub struct Node {
  out_edges: Option<Vec<Arc<Mutex<Edge>>>>,
  through_value: Option<f32>,
  threshold: f32,
}

impl Default for Node {
  fn default() -> Self {
    Self {
      out_edges: None,
      through_value: None,
      threshold: 1.0
    }
  }
}

impl NetAspect for Node {
  fn give_value(&mut self, value: f32) {
    self.through_value = Some(value);
  }
}

impl Node {
  pub fn fire(&mut self, through_value: f32) {
    self.give_value(through_value);
    if self.activate() {
      self.send_value();
    }
  }

  pub fn get_through_value(&self) -> f32 {
    self.through_value.unwrap()
  }

  fn activate(&self) -> bool {
    self.through_value.unwrap() >= self.threshold
  }
  
  fn send_value(&self) {
    self.out_edges.unwrap().iter()
      .for_each(|edge| 
                  { ***edge.borrow_mut() }.lock().unwrap()
                    .fire(self.through_value.unwrap())
                );
  }
}
  