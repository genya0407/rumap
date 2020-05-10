use crate::*;
use std::cell::RefCell;

#[derive(Clone)]
pub struct MockShellCommandExecutor {
  pub arguments: RefCell<Vec<String>>,
}

impl MockShellCommandExecutor {
  pub fn new() -> Self {
    Self {
      arguments: RefCell::new(vec![]),
    }
  }
}

impl IsShellCommandExecutor<String> for MockShellCommandExecutor {
  fn execute(&self, command: String) {
    self.arguments.borrow_mut().push(command);
  }
}
