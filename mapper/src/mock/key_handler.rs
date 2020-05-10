use crate::*;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MockKeyHandler {
  pub pressed_keys: Rc<Mutex<Vec<KeyInput<String, String>>>>,
  pub released_keys: Rc<Mutex<Vec<KeyInput<String, String>>>>,
}

impl MockKeyHandler {
  pub fn new() -> Self {
    Self {
      pressed_keys: Rc::new(Mutex::new(vec![])),
      released_keys: Rc::new(Mutex::new(vec![])),
    }
  }
}

impl IsKeyHandler<String, String> for MockKeyHandler {
  fn press_key(&self, key_input: KeyInput<String, String>) {
    self.pressed_keys.lock().unwrap().push(key_input);
  }

  fn release_key(&self, key_input: KeyInput<String, String>) {
    self.released_keys.lock().unwrap().push(key_input);
  }
}
