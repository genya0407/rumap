use super::{KeyInput, XExecution, XKeySymbol, XModifier};
use mapper::{Action, KeyBind};

#[derive(Debug, Clone)]
pub struct XExecutionKeyBind {
  from: KeyInput,
  execution: XExecution,
}

impl XExecutionKeyBind {
  pub fn new(from: KeyInput, command: String) -> Self {
    Self {
      from: from,
      execution: XExecution::ShellCommand(command),
    }
  }
}

impl KeyBind<XKeySymbol, XModifier, XExecution> for XExecutionKeyBind {
  fn pressed(&self, key_input: &KeyInput) -> Option<Action<XKeySymbol, XModifier, XExecution>> {
    if *key_input == self.from {
      Some(Action::Execution {
        execution: self.execution.clone(),
      })
    } else {
      None
    }
  }

  fn released(&self, _key_input: &KeyInput) -> Option<Action<XKeySymbol, XModifier, XExecution>> {
    None
  }
}
