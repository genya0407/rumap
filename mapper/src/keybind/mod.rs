use super::Action;
use super::KeyInput;

pub mod composit_keybind;
pub mod keybind_for_focus;

pub use composit_keybind::*;
pub use keybind_for_focus::*;

// keyinputにActionを関連付けるもの
pub trait KeyBind<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  C: std::fmt::Debug + Clone,
>
{
  fn pressed(&self, key_input: &KeyInput<K, M>) -> Option<Action<K, M, C>>;
  fn released(&self, key_input: &KeyInput<K, M>) -> Option<Action<K, M, C>>;
}
