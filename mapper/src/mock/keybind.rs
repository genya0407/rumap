pub struct MockKeyBind {
  pub from: String,
  pub execution: String,
}

impl crate::KeyBind<String, String, String> for MockKeyBind {
  fn pressed(
    &self,
    key_input: &crate::KeyInput<String, String>,
  ) -> Option<crate::Action<String, String, String>> {
    log::debug!("{:?}", key_input);
    if self.from == key_input.key().raw_value() {
      Some(crate::Action::Execution {
        execution: format!("pressed/{}", self.execution.clone()),
      })
    } else {
      None
    }
  }

  fn released(
    &self,
    key_input: &crate::KeyInput<String, String>,
  ) -> Option<crate::Action<String, String, String>> {
    log::debug!("{:?}", key_input);
    if self.from == key_input.key().raw_value() {
      Some(crate::Action::Execution {
        execution: format!("released/{}", self.execution.clone()),
      })
    } else {
      None
    }
  }
}
