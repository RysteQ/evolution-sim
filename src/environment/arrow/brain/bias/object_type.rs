use crate::environment::arena::modifier::Modifier;

#[derive(Clone)]
pub enum ObjectType {
  PowerupObject(Modifier),
  ArrowObject,
}
