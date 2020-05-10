use crate::*;

#[derive(Clone)]
pub enum Event<
  K: PartialEq + Eq + Clone + std::fmt::Debug,
  M: PartialOrd + Ord + Clone + std::fmt::Debug,
  A: PartialEq + Eq + PartialOrd + Ord + Clone,
> {
  KeyPressed {
    key_input: KeyInput<K, M>,
  },
  KeyReleased {
    key_input: KeyInput<K, M>,
  },
  ApplicationChanged {
    next_application: Option<Application<A>>,
  },
}
