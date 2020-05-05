use crate::*;

pub struct XIntoDomain;

impl<'a> mapper::config::IsIntoDomain<'a, XAppIdentifier, XKeySymbol, XModifier, XExecution>
  for XIntoDomain
{
  fn into_domain_application(
    &self,
    app: mapper::config::Application,
  ) -> Result<Application, mapper::config::InvalidConfigError> {
    Ok(Application::new(app.0))
  }

  fn into_domain_keyinput(
    &self,
    key_input: mapper::config::KeyInput,
  ) -> Result<KeyInput, mapper::config::InvalidConfigError> {
    // Modifier-Modifier-...-Key となっているのをパースする
    let key_input = key_input.0;
    let mut key_names = key_input.split('-').collect::<Vec<&str>>();
    if key_names.len() == 0 {
      return Err(mapper::config::InvalidConfigError::EmptyKey);
    }

    let key_name = key_names.pop().unwrap();
    let modifier_names = key_names;

    let key = self.into_domain_key(mapper::config::Key(key_name.to_string()))?;

    let mut modifier_masks = vec![];
    for modifier_name in modifier_names {
      modifier_masks
        .push(self.into_domain_modifier(mapper::config::Modifier(modifier_name.to_string()))?);
    }
    let modifiers = Modifiers::new(modifier_masks);

    Ok(KeyInput::new(key, modifiers))
  }

  fn into_domain_action(
    &self,
    from: mapper::config::KeyInput,
    execution: mapper::config::Execution,
  ) -> Result<
    Box<dyn mapper::KeyBind<XKeySymbol, XModifier, XExecution> + 'a>,
    mapper::config::InvalidConfigError,
  > {
    Ok(Box::new(XExecutionKeyBind::new(
      self.into_domain_keyinput(from)?,
      execution.0,
    )))
  }

  fn into_domain_key(
    &self,
    key: mapper::config::Key,
  ) -> Result<crate::Key, mapper::config::InvalidConfigError> {
    super::keysyms::KEYNAME_TO_KEYSYM
      .get(key.0.as_str())
      .cloned()
      .map(mapper::Key::new)
      .ok_or(mapper::config::InvalidConfigError::UnexpectedKey(key.0))
  }

  fn into_domain_modifier(
    &self,
    modifier: mapper::config::Modifier,
  ) -> Result<crate::Modifier, mapper::config::InvalidConfigError> {
    super::keysyms::MODIFIERNAME_TO_MASK
      .get(modifier.0.as_str())
      .cloned()
      .map(mapper::Modifier::new)
      .ok_or(mapper::config::InvalidConfigError::UnexpectedModifier(
        modifier.0,
      ))
  }
}
