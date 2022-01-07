mod vision;
mod decisions;

use crate::environment::arrow::brain::vision::Vision;
use crate::environment::arrow::brain::decisions::Decisions;
use crate::environment::arrow::direction::Direction;

#[derive(Default, Clone)]
pub struct Brain {
  viewing: Vision,
  decisions: Decisions,
  snapshots: Vec<(Vision, Direction)>,
}

impl Brain {
  pub fn make_decision(&self) -> Direction {
    self.decisions.make_decision()
  }
}
