use super::{Action, KeyBind, KeyInput, Matching};

// keyinput -> keyinput というような対応を取るkeybind
pub struct Keymap<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
> {
  from: KeyInput<K, M>,
  to: KeyInput<K, M>,
}

impl<K: PartialEq + Eq + std::fmt::Debug + Clone, M: PartialOrd + Ord + std::fmt::Debug + Clone>
  Keymap<K, M>
{
  pub fn new(from: KeyInput<K, M>, to: KeyInput<K, M>) -> Self {
    log::debug!("Keymap initialized: {:?} -> {:?}", from, to);
    Self { from: from, to: to }
  }
}

impl<
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
  > KeyBind<K, M, C> for Keymap<K, M>
{
  fn pressed(&self, key_input: &KeyInput<K, M>) -> Option<Action<K, M, C>> {
    log::debug!("{:?}", self.from.match_to(&key_input));
    match self.from.match_to(&key_input) {
      Matching::Unmatched => None,
      Matching::Remain(modifiers) => {
        let remapped_key_input = self.to.merge_modifiers(&modifiers);
        log::debug!("{:?}", remapped_key_input);
        Some(Action::Key {
          key_input: remapped_key_input,
        })
      }
    }
  }

  fn released(&self, key_input: &KeyInput<K, M>) -> Option<Action<K, M, C>> {
    log::debug!("{:?}", self.from.match_to(&key_input));
    match self.from.match_to(&key_input) {
      Matching::Unmatched => None,
      Matching::Remain(modifiers) => {
        let remapped_key_input = self.to.merge_modifiers(&modifiers);
        log::debug!("{:?}", remapped_key_input);
        Some(Action::Key {
          key_input: remapped_key_input,
        })
      }
    }
  }
}
