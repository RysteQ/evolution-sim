use crate::environment::arena::modifier::Modifier;

#[derive(Clone, Debug)]
pub enum ObjectType {
  PowerupObject(Modifier),
  ArrowObject,
}
