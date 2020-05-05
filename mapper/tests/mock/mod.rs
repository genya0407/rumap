use mapper::config::IsIntoDomain;

pub struct TestAction {
  pub from: String,
  pub execution: String,
}

impl mapper::KeyBind<String, String, String> for TestAction {
  fn pressed(
    &self,
    key_input: &mapper::KeyInput<String, String>,
  ) -> Option<mapper::Action<String, String, String>> {
    log::debug!("{:?}", key_input);
    if self.from == key_input.key().raw_value() {
      Some(mapper::Action::Execution {
        execution: format!("pressed/{}", self.execution.clone()),
      })
    } else {
      None
    }
  }

  fn released(
    &self,
    key_input: &mapper::KeyInput<String, String>,
  ) -> Option<mapper::Action<String, String, String>> {
    log::debug!("{:?}", key_input);
    if self.from == key_input.key().raw_value() {
      Some(mapper::Action::Execution {
        execution: format!("released/{}", self.execution.clone()),
      })
    } else {
      None
    }
  }
}

pub struct StringIntoDomain;

impl<'a> IsIntoDomain<'a, String, String, String, String> for StringIntoDomain {
  fn into_domain_application(
    &self,
    app: mapper::config::Application,
  ) -> Result<mapper::Application<String>, mapper::config::InvalidConfigError> {
    Ok(mapper::Application::new(app.0))
  }

  fn into_domain_keyinput(
    &self,
    key_input: mapper::config::KeyInput,
  ) -> Result<mapper::KeyInput<String, String>, mapper::config::InvalidConfigError> {
    Ok(mapper::KeyInput::new(
      mapper::Key::new(key_input.0),
      mapper::Modifiers::new(vec![]),
    ))
  }

  fn into_domain_action(
    &self,
    from: mapper::config::KeyInput,
    execution: mapper::config::Execution,
  ) -> Result<
    Box<dyn mapper::KeyBind<String, String, String> + 'a>,
    mapper::config::InvalidConfigError,
  > {
    log::debug!("TestAction initialized: {:?} -> {:?}", from, execution);
    Ok(Box::new(TestAction {
      from: from.0,
      execution: execution.0,
    }))
  }

  fn into_domain_key(
    &self,
    key: mapper::config::Key,
  ) -> Result<mapper::Key<String>, mapper::config::InvalidConfigError> {
    Ok(mapper::Key::new(key.0))
  }

  fn into_domain_modifier(
    &self,
    modifier: mapper::config::Modifier,
  ) -> Result<mapper::Modifier<String>, mapper::config::InvalidConfigError> {
    Ok(mapper::Modifier::new(modifier.0))
  }
}
