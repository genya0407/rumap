use linux::*;

pub struct TestAction {
  pub from: KeyInput,
  pub execution: XExecution,
}

impl mapper::KeyBind<XKeySymbol, XModifier, XExecution> for TestAction {
  fn pressed(
    &self,
    key_input: &mapper::KeyInput<XKeySymbol, XModifier>,
  ) -> Option<mapper::Action<XKeySymbol, XModifier, XExecution>> {
    log::debug!("{:?}", key_input);
    if &self.from == key_input {
      Some(mapper::Action::Execution {
        execution: self.execution.clone(),
      })
    } else {
      None
    }
  }

  fn released(
    &self,
    key_input: &mapper::KeyInput<XKeySymbol, XModifier>,
  ) -> Option<mapper::Action<XKeySymbol, XModifier, XExecution>> {
    log::debug!("{:?}", key_input);
    if &self.from == key_input {
      Some(mapper::Action::Execution {
        execution: self.execution.clone(),
      })
    } else {
      None
    }
  }
}
