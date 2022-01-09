// 1-layer thick matrices because I'm not feeling like implementing a proper matrix nor do I need to for this project

type MatrixValues = Vec<Option<f32>>;

#[derive(Clone, Debug)]
pub struct Layer(MatrixValues);

impl Layer {
  pub fn from_vec(vec: Vec<Option<f32>>) -> Self {
    Self(vec)
  }

  pub fn with_size(size: usize, value: Option<f32>) -> Self {
    Self({
      let mut out_vec = Vec::with_capacity(size);
      for _ in 0..size {
        out_vec.push(value);
      }
      out_vec
    })
  }

  pub fn get_values(&self) -> &MatrixValues {
    &self.0
  }

  pub fn set_element(&mut self, i: usize, new_value: Option<f32>) {
    *self.0.get_mut(i).unwrap() = new_value;
  }

  pub fn set_value(&mut self, new_value: Vec<Option<f32>>) {
    self.0 = new_value;
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn get(&self, i: usize) -> Option<&f32> {
    self.0.get(i).unwrap().as_ref()
  }

  pub fn multiply(&self, other: &Self) -> Self {
    assert_eq!(self.len(), other.len());
    
    let mut out_vec = Vec::with_capacity(self.len());

    for i in 0..self.len() {
      out_vec.push(Some({
        self.get(i).unwrap() * other.get(i).unwrap()
      }));
    }

    Self::from_vec(out_vec)
  }
}
