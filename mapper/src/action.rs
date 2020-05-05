use super::KeyInput;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  C: std::fmt::Debug + Clone,
> {
  Key { key_input: KeyInput<K, M> },
  Execution { execution: C },
}
