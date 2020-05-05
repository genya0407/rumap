use crate::Action;
use crate::KeyBind;
use crate::KeyInput;

pub struct CompositKeyBind<
  'a,
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  C: std::fmt::Debug + Clone,
> {
  keybindings: Vec<Box<dyn KeyBind<K, M, C> + 'a>>,
}

impl<
    'a,
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
  > CompositKeyBind<'a, K, M, C>
{
  pub fn new(keybindings: Vec<Box<dyn KeyBind<K, M, C> + 'a>>) -> Self {
    Self {
      keybindings: keybindings,
    }
  }
}

impl<
    'a,
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
  > KeyBind<K, M, C> for CompositKeyBind<'a, K, M, C>
{
  fn pressed(&self, key_input: &KeyInput<K, M>) -> Option<Action<K, M, C>> {
    for keybinding in self.keybindings.iter() {
      if let Some(action) = keybinding.pressed(&key_input) {
        // 一つでもbindに成功したら終了
        return Some(action);
      }
    }

    return None;
  }

  fn released(&self, key_input: &KeyInput<K, M>) -> Option<Action<K, M, C>> {
    for keybinding in self.keybindings.iter() {
      if let Some(action) = keybinding.released(&key_input) {
        // 一つでもbindに成功したら終了
        return Some(action);
      }
    }

    return None;
  }
}
