use linux::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct MockKeyBindForFocus {
  expect_pressed: BTreeMap<(Focus, KeyInput), Action>,
  expect_released: BTreeMap<(Focus, KeyInput), Action>,
  pub pressed: RefCell<Vec<String>>,
  pub released: RefCell<Vec<String>>,
}

impl MockKeyBindForFocus {
  pub fn new(
    expect_pressed: BTreeMap<(Focus, KeyInput), Action>,
    expect_released: BTreeMap<(Focus, KeyInput), Action>,
  ) -> Self {
    Self {
      expect_pressed,
      expect_released,
      pressed: RefCell::new(vec![]),
      released: RefCell::new(vec![]),
    }
  }
}

impl mapper::IsKeyBindForFocus<XAppIdentifier, XKeySymbol, XModifier, XExecution>
  for MockKeyBindForFocus
{
  fn pressed(&self, focus: Focus, key_input: KeyInput) -> Option<Action> {
    self.expect_pressed.get(&(focus, key_input)).cloned()
  }

  fn released(&self, focus: Focus, key_input: KeyInput) -> Option<Action> {
    self.expect_released.get(&(focus, key_input)).cloned()
  }
}
