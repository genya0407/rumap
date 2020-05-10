use crate::*;

pub trait IsEventSource<
  K: PartialEq + Eq + Clone + std::fmt::Debug,
  M: PartialOrd + Ord + Clone + std::fmt::Debug,
  A: PartialEq + Eq + PartialOrd + Ord + Clone,
>
{
  fn ungrab_keys(&self);
  fn grab_keys(&self, key_inputs: Vec<KeyInput<K, M>>);
  fn next(&self) -> Option<Event<K, M, A>>;
}
