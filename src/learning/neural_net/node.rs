use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use crate::learning::neural_net::edge::Edge;
use crate::learning::neural_net::net_aspect::NetAspect;

#[derive(Clone, Debug)]
pub struct Node {
  out_edges: Option<Vec<Arc<Mutex<Edge>>>>,  // sheesh
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

  pub fn default_with_edges(edges: Vec<Edge>) -> Self {
    let mut default_node = Self::default();
    default_node.give_edges(edges);
    default_node
  }

  pub fn give_edges(&mut self, edges: Vec<Edge>) {
    self.out_edges = Some({
      let mut out_vec = Vec::new();
      for edge in edges {
        out_vec.push(Arc::new(Mutex::new(edge)));
      }
      out_vec
    })
  }

  pub fn get_through_value(&self) -> f32 {
    self.through_value.unwrap()
  }

  fn activate(&self) -> bool {
    self.through_value.unwrap() >= self.threshold
  }
  
  fn send_value(&mut self) {
    self.out_edges.as_ref().unwrap().iter()
      .for_each(|mut edge| 
                  edge.borrow_mut().lock().unwrap()
                    .fire(self.through_value.unwrap())
                );
  }
}
  