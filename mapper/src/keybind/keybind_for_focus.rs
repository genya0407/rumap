use super::KeyBind;
use crate::Action;
use crate::Application;
use crate::Focus;
use crate::KeyInput;
use std::collections::BTreeMap;

pub trait IsKeyBindForFocus<
  A: PartialEq + Eq + PartialOrd + Ord + Clone,
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  C: std::fmt::Debug + Clone,
>
{
  fn pressed(&self, focus: Focus<A>, key_input: KeyInput<K, M>) -> Option<Action<K, M, C>>;
  fn released(&self, focus: Focus<A>, key_input: KeyInput<K, M>) -> Option<Action<K, M, C>>;
}

pub struct KeyBindForFocus<
  'a,
  A: PartialEq + Eq + PartialOrd + Ord + Clone,
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  C: std::fmt::Debug + Clone,
> {
  global_keybind: Box<dyn KeyBind<K, M, C> + 'a>,
  keybind_by_application: BTreeMap<Application<A>, Box<dyn KeyBind<K, M, C> + 'a>>,
}

impl<
    'a,
    A: PartialEq + Eq + PartialOrd + Ord + Clone,
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
  > KeyBindForFocus<'a, A, K, M, C>
{
  pub fn new(
    global_keybind: Box<dyn KeyBind<K, M, C> + 'a>,
    keybind_by_application: BTreeMap<Application<A>, Box<dyn KeyBind<K, M, C> + 'a>>,
  ) -> Self {
    Self {
      global_keybind: global_keybind,
      keybind_by_application: keybind_by_application,
    }
  }
}

impl<
    'a,
    A: PartialEq + Eq + PartialOrd + Ord + Clone,
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
  > IsKeyBindForFocus<A, K, M, C> for KeyBindForFocus<'a, A, K, M, C>
{
  fn pressed(&self, focus: Focus<A>, key_input: KeyInput<K, M>) -> Option<Action<K, M, C>> {
    match focus {
      Focus::NoFocus => self.global_keybind.pressed(&key_input),
      Focus::Focused { application: app } => {
        // まずapplicationのkeybindを見に行き、該当しなければglobalのkeybindを見る
        self
          .keybind_by_application
          .get(&app)
          .and_then(|keybind| keybind.pressed(&key_input))
          .or(self.global_keybind.pressed(&key_input))
      }
    }
  }

  fn released(&self, focus: Focus<A>, key_input: KeyInput<K, M>) -> Option<Action<K, M, C>> {
    match focus {
      Focus::NoFocus => self.global_keybind.released(&key_input),
      Focus::Focused { application: app } => {
        // まずapplicationのkeybindを見に行き、該当しなければglobalのkeybindを見る
        self
          .keybind_by_application
          .get(&app)
          .and_then(|keybind| keybind.released(&key_input))
          .or(self.global_keybind.released(&key_input))
      }
    }
  }
}
