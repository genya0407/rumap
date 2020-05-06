use std::rc::Rc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MockKeyHandler {
  pub pressed_keys: Rc<Mutex<Vec<linux::KeyInput>>>,
  pub released_keys: Rc<Mutex<Vec<linux::KeyInput>>>,
}

impl MockKeyHandler {
  pub fn new() -> Self {
    Self {
      pressed_keys: Rc::new(Mutex::new(vec![])),
      released_keys: Rc::new(Mutex::new(vec![])),
    }
  }
}

impl linux::IsKeyHandler for MockKeyHandler {
  fn press_key(&self, key_input: linux::KeyInput) {
    self.pressed_keys.lock().unwrap().push(key_input);
  }

  fn release_key(&self, key_input: linux::KeyInput) {
    self.released_keys.lock().unwrap().push(key_input);
  }
}
