// 1-layer thick matrices because I'm not feeling like implementing a proper matrix nor do I need to for this project

type MatrixValues = Vec<f32>;

#[derive(Clone)]
pub struct NetMatrix(MatrixValues);

impl NetMatrix {
  pub fn from_vec(vec: MatrixValues) -> Self {
    Self(vec)
  }

  pub fn with_size(size: usize) -> Self {
    Self({
      let mut out_vec = Vec::with_capacity(size);
      for _ in 0..size {
        out_vec.push(0.0);
      }
      out_vec
    })
  }

  pub fn get_values(&self) -> MatrixValues {
    self.0
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn get(&self, i: usize) -> Option<&f32> {
    self.0.get(i)
  }

  pub fn multiply(&self, other: Self) -> Self {
    assert_eq!(self.len(), other.len());
    
    let mut out_vec = Vec::with_capacity(self.len());

    for i in 0..self.len() {
      out_vec.push({
        self.get(i).unwrap() * other.get(i).unwrap()
      });
    }

    Self::from_vec(out_vec)
  }
}
