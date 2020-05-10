use crate::config::IsIntoDomain;

pub struct StringIntoDomain;

impl<'a> IsIntoDomain<'a, String, String, String, String> for StringIntoDomain {
  fn into_domain_application(
    &self,
    app: crate::config::Application,
  ) -> Result<crate::Application<String>, crate::config::InvalidConfigError> {
    Ok(crate::Application::new(app.0))
  }

  fn into_domain_keyinput(
    &self,
    key_input: crate::config::KeyInput,
  ) -> Result<crate::KeyInput<String, String>, crate::config::InvalidConfigError> {
    Ok(crate::KeyInput::new(
      crate::Key::new(key_input.0),
      crate::Modifiers::new(vec![]),
    ))
  }

  fn into_domain_action(
    &self,
    from: crate::config::KeyInput,
    execution: crate::config::Execution,
  ) -> Result<Box<dyn crate::KeyBind<String, String, String> + 'a>, crate::config::InvalidConfigError>
  {
    log::debug!("MockKeyBind initialized: {:?} -> {:?}", from, execution);
    Ok(Box::new(crate::mock::MockKeyBind {
      from: from.0,
      execution: execution.0,
    }))
  }

  fn into_domain_key(
    &self,
    key: crate::config::Key,
  ) -> Result<crate::Key<String>, crate::config::InvalidConfigError> {
    Ok(crate::Key::new(key.0))
  }

  fn into_domain_modifier(
    &self,
    modifier: crate::config::Modifier,
  ) -> Result<crate::Modifier<String>, crate::config::InvalidConfigError> {
    Ok(crate::Modifier::new(modifier.0))
  }
}
