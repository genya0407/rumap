pub trait IsIntoDomain<
  'a,
  A: PartialEq + Eq + PartialOrd + Ord + Clone + 'a,
  K: PartialEq + Eq + std::fmt::Debug + Clone + 'a,
  M: PartialOrd + Ord + std::fmt::Debug + Clone + 'a,
  C: std::fmt::Debug + Clone + 'a,
>
{
  fn into_domain_application(
    &self,
    app: super::Application,
  ) -> Result<crate::Application<A>, super::InvalidConfigError>;

  fn into_domain_keyinput(
    &self,
    key_input: super::KeyInput,
  ) -> Result<crate::KeyInput<K, M>, super::InvalidConfigError>;

  fn into_domain_action(
    &self,
    from: super::KeyInput,
    execution: super::Execution,
  ) -> Result<Box<dyn crate::KeyBind<K, M, C> + 'a>, super::InvalidConfigError>;

  fn into_domain_key(&self, key: super::Key) -> Result<crate::Key<K>, super::InvalidConfigError>;

  fn into_domain_modifier(
    &self,
    modifier: super::Modifier,
  ) -> Result<crate::Modifier<M>, super::InvalidConfigError>;
}
