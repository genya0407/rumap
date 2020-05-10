use crate::*;

pub trait IsKeyHandler<
  K: PartialEq + Eq + Clone + std::fmt::Debug,
  M: PartialOrd + Ord + Clone + std::fmt::Debug,
>
{
  fn press_key(&self, key_input: KeyInput<K, M>);
  fn release_key(&self, key_input: KeyInput<K, M>);
}
