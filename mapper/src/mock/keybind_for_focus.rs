use crate::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct MockKeyBindForFocus {
  expect_pressed:
    BTreeMap<(Focus<String>, KeyInput<String, String>), Action<String, String, String>>,
  expect_released:
    BTreeMap<(Focus<String>, KeyInput<String, String>), Action<String, String, String>>,
  pub pressed: RefCell<Vec<String>>,
  pub released: RefCell<Vec<String>>,
}

impl MockKeyBindForFocus {
  pub fn new(
    expect_pressed: BTreeMap<
      (Focus<String>, KeyInput<String, String>),
      Action<String, String, String>,
    >,
    expect_released: BTreeMap<
      (Focus<String>, KeyInput<String, String>),
      Action<String, String, String>,
    >,
  ) -> Self {
    Self {
      expect_pressed,
      expect_released,
      pressed: RefCell::new(vec![]),
      released: RefCell::new(vec![]),
    }
  }
}

impl IsKeyBindForFocus<String, String, String, String> for MockKeyBindForFocus {
  fn pressed(
    &self,
    focus: Focus<String>,
    key_input: KeyInput<String, String>,
  ) -> Option<Action<String, String, String>> {
    self.expect_pressed.get(&(focus, key_input)).cloned()
  }

  fn released(
    &self,
    focus: Focus<String>,
    key_input: KeyInput<String, String>,
  ) -> Option<Action<String, String, String>> {
    self.expect_released.get(&(focus, key_input)).cloned()
  }
}
