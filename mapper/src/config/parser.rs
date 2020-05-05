use super::*;
use crate::*;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub trait IsParser<
  'a,
  A: PartialEq + Eq + PartialOrd + Ord + Clone + 'a,
  K: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug + Clone + 'a,
  M: PartialOrd + Ord + std::fmt::Debug + Clone + 'a,
  C: std::fmt::Debug + Clone + 'a,
>
{
  fn build_keybind_for_focus(
    &self,
    config: config::Config,
  ) -> Result<KeyBindForFocus<'a, A, K, M, C>, InvalidConfigError>;
  fn build_possible_keyinput_finder(
    &self,
    config: config::Config,
  ) -> Result<PossibleKeyinputFinder<A, K, M>, InvalidConfigError>;
}

pub struct Parser<
  'a,
  A: PartialEq + Eq + PartialOrd + Ord + Clone + 'a,
  K: PartialEq + Eq + std::fmt::Debug + Clone + 'a,
  M: PartialOrd + Ord + std::fmt::Debug + Clone + 'a,
  C: std::fmt::Debug + Clone + 'a,
  ID: IsIntoDomain<'a, A, K, M, C>,
> {
  _a: PhantomData<A>,
  _k: PhantomData<K>,
  _m: PhantomData<M>,
  _c: PhantomData<C>,
  into_domain: &'a ID,
}

impl<
    'a,
    A: PartialEq + Eq + PartialOrd + Ord + Clone + 'a,
    K: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug + Clone + 'a,
    M: PartialOrd + Ord + std::fmt::Debug + Clone + 'a,
    C: std::fmt::Debug + Clone + 'a,
    ID: IsIntoDomain<'a, A, K, M, C>,
  > IsParser<'a, A, K, M, C> for Parser<'a, A, K, M, C, ID>
{
  fn build_keybind_for_focus(
    &self,
    config: config::Config,
  ) -> Result<KeyBindForFocus<'a, A, K, M, C>, InvalidConfigError> {
    let global_keybinds: Box<dyn KeyBind<K, M, C> + 'a> = Box::new(CompositKeyBind::new(
      config
        .remap
        .0
        .into_iter()
        .map(|(from, action)| self.remap_to_keybind(from, action))
        .collect::<Result<_, _>>()?,
    ));

    let mut keybinds_for_applications: BTreeMap<
      crate::Application<A>,
      Box<dyn KeyBind<K, M, C> + 'a>,
    > = BTreeMap::new();
    for (app, remaps) in config.in_app.into_iter() {
      let keybinds: Vec<Box<dyn KeyBind<K, M, C> + 'a>> = remaps
        .0
        .into_iter()
        .map(|(from, action)| self.remap_to_keybind(from, action))
        .collect::<Result<_, _>>()?;
      let composit_keybind: Box<dyn KeyBind<K, M, C> + 'a> =
        Box::new(CompositKeyBind::new(keybinds));

      keybinds_for_applications.insert(
        self.into_domain.into_domain_application(app)?,
        composit_keybind,
      );
    }

    Ok(KeyBindForFocus::new(
      global_keybinds,
      keybinds_for_applications,
    ))
  }

  fn build_possible_keyinput_finder(
    &self,
    config: config::Config,
  ) -> Result<PossibleKeyinputFinder<A, K, M>, InvalidConfigError> {
    let mut cache = BTreeMap::<crate::Focus<A>, Vec<crate::KeyInput<K, M>>>::new();

    // global remap
    for (from_config, action) in config.remap.0.into_iter() {
      let possible_modifiers = match action {
        config::values::Action::KeyInput { to: _, with } => with.unwrap_or_default(),
        _ => continue,
      };
      let possible_keyinputs =
        self.possible_modifiers_to_keyinputs(from_config, possible_modifiers)?;
      cache
        .entry(Focus::NoFocus)
        .and_modify(|keyinputs| keyinputs.extend(possible_keyinputs.clone()))
        .or_insert(possible_keyinputs);
    }

    // application specific remap
    for (app, remap) in config.in_app.into_iter() {
      let focus = Focus::Focused {
        application: self.into_domain.into_domain_application(app)?,
      };
      for (from_config, action) in remap.0.clone().into_iter() {
        let possible_modifiers = match action {
          config::values::Action::KeyInput { to: _, with } => with.unwrap_or_default(),
          _ => continue,
        };
        let possible_keyinputs =
          self.possible_modifiers_to_keyinputs(from_config, possible_modifiers)?;
        cache
          .entry(focus.clone())
          .and_modify(|keyinputs| keyinputs.extend(possible_keyinputs.clone()))
          .or_insert(possible_keyinputs);
      }
    }

    Ok(PossibleKeyinputFinder::new(cache))
  }
}

impl<
    'a,
    A: PartialEq + Eq + PartialOrd + Ord + Clone + 'a,
    K: PartialEq + Eq + std::fmt::Debug + Clone + 'a,
    M: PartialOrd + Ord + std::fmt::Debug + Clone + 'a,
    C: std::fmt::Debug + Clone + 'a,
    ID: IsIntoDomain<'a, A, K, M, C>,
  > Parser<'a, A, K, M, C, ID>
{
  pub fn build(id: &'a ID) -> Self {
    Self {
      _a: PhantomData,
      _k: PhantomData,
      _m: PhantomData,
      _c: PhantomData,
      into_domain: id,
    }
  }

  fn possible_modifiers_to_keyinputs(
    &self,
    base: super::KeyInput,
    possible_modifiers: Vec<super::Modifier>,
  ) -> Result<Vec<crate::KeyInput<K, M>>, InvalidConfigError> {
    let base = self.into_domain.into_domain_keyinput(base)?;
    let mut keyinputs = vec![];
    for modifier_length in 0..=possible_modifiers.len() {
      for additional_modifiers in possible_modifiers
        .clone()
        .into_iter()
        .combinations(modifier_length)
      {
        keyinputs.push(
          base.clone().merge_modifiers(&Modifiers::new(
            additional_modifiers
              .into_iter()
              .map(|m| self.into_domain.into_domain_modifier(m))
              .collect::<Result<Vec<crate::Modifier<M>>, _>>()?,
          )),
        )
      }
    }
    Ok(keyinputs)
  }

  fn remap_to_keybind(
    &self,
    from: super::KeyInput,
    action: super::Action,
  ) -> Result<Box<dyn KeyBind<K, M, C> + 'a>, InvalidConfigError> {
    match action {
      super::Action::KeyInput { to, with: _ } => Ok(Box::new(crate::Keymap::new(
        self.into_domain.into_domain_keyinput(from)?,
        self.into_domain.into_domain_keyinput(to)?,
      ))),
      super::Action::Execution { execute: execution } => {
        self.into_domain.into_domain_action(from, execution)
      }
    }
  }
}
