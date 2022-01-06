mod vision;
mod decisions;

use crate::environment::arrow::brain::vision::Vision;
use crate::environment::arrow::brain::decisions::Decisions;

pub struct Brain {
  viewing: Vision,
  decisions: Decisions,
}
