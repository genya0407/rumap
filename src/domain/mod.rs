use std::collections::{BTreeMap, BTreeSet};

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Application<A: PartialOrd + Ord + PartialEq + Eq> {
  pub name: A,
}

#[derive(Debug)]
pub enum Matching<M> {
  Unmatched,
  Remain(Modifiers<M>),
}

#[derive(Clone, Debug)]
pub struct KeyInput<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
> {
  key: Key<K>,
  modifiers: Modifiers<M>,
}

impl<K: PartialEq + Eq + std::fmt::Debug + Clone, M: PartialOrd + Ord + std::fmt::Debug + Clone>
  KeyInput<K, M>
{
  pub fn new(key: Key<K>, modifiers: Modifiers<M>) -> Self {
    Self {
      key: key,
      modifiers: modifiers,
    }
  }

  pub fn match_to(&self, target: &Self) -> Matching<M> {
    if self.key == target.key {
      if let ModifiersSub::SubResult(remaining_modifiers) =
        target.modifiers.subtract_modifiers(&self.modifiers)
      {
        Matching::Remain(remaining_modifiers)
      } else {
        Matching::Unmatched
      }
    } else {
      Matching::Unmatched
    }
  }

  pub fn merge_modifiers(&self, modifiers: &Modifiers<M>) -> Self {
    Self {
      key: self.key.clone(),
      modifiers: self.modifiers.merge(modifiers),
    }
  }

  pub fn key(&self) -> Key<K> {
    self.key.clone()
  }

  pub fn modifiers(&self) -> Modifiers<M> {
    self.modifiers.clone()
  }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Key<K: PartialEq + Eq + Clone> {
  value: K,
}
impl<K: PartialEq + Eq + Clone> Key<K> {
  pub fn new(value: K) -> Self {
    Self { value: value }
  }

  pub fn raw_value(&self) -> K {
    self.value.clone()
  }
}

enum ModifiersSub<M> {
  IsNotSubset,
  SubResult(Modifiers<M>),
}

#[derive(Clone, Debug)]
pub struct Modifiers<M> {
  value: BTreeSet<Modifier<M>>,
}

impl<M: PartialOrd + Ord + Clone> Modifiers<M> {
  pub fn new(modifiers: Vec<Modifier<M>>) -> Self {
    let mut set = BTreeSet::new();
    for modifier in modifiers.into_iter() {
      set.insert(modifier);
    }
    Self { value: set }
  }

  // selfからtargetをとりのぞく
  fn subtract_modifiers(&self, target: &Self) -> ModifiersSub<M> {
    use std::ops::Sub;

    if target.value.is_subset(&self.value) {
      ModifiersSub::SubResult(Self {
        value: self.value.sub(&target.value),
      })
    } else {
      ModifiersSub::IsNotSubset
    }
  }

  pub fn merge(&self, target: &Self) -> Self {
    Self {
      value: &self.value | &target.value,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.value.is_empty()
  }

  pub fn to_vec(&self) -> Vec<Modifier<M>> {
    self.value.clone().into_iter().collect()
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Modifier<M> {
  value: M,
}
impl<M: Clone> Modifier<M> {
  pub fn new(value: M) -> Self {
    Self { value: value }
  }

  pub fn raw_value(&self) -> M {
    self.value.clone()
  }
}

pub enum Event<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
> {
  KeyPressed(KeyInput<K, M>),
  KeyReleased(KeyInput<K, M>),
  ApplicationChange(Application<APP>),
}

pub trait EventSource<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
>
{
  fn remap_keys(&self) -> Result<(), Box<dyn std::error::Error>> {
    self.initialize_register_state()?;
    for key_input in self.watch_target_key_inputs() {
      self.register_key(key_input)?;
    }
    Ok(())
  }
  fn initialize_register_state(&self) -> Result<(), Box<dyn std::error::Error>>;
  fn register_key(&self, key_input: KeyInput<K, M>) -> Result<(), Box<dyn std::error::Error>>;
  fn next(&self) -> Option<Event<K, M, APP>>;
  fn watch_target_key_inputs(&self) -> Vec<KeyInput<K, M>>;
}

pub trait IsEventHandler<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
>
{
  fn key_press(&self, key_input: KeyInput<K, M>);
  fn key_release(&self, key_input: KeyInput<K, M>);
  fn change_application(&mut self, application: Application<APP>);
}

pub struct EventWatcher<
  ES: EventSource<K, M, APP>,
  EH: IsEventHandler<K, M, APP>,
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
> {
  event_source: ES,
  event_handler: EH,
  _raw_key_type: std::marker::PhantomData<K>,
  _raw_modifier_type: std::marker::PhantomData<M>,
  _raw_app_identifier_type: std::marker::PhantomData<APP>,
}

impl<
    ES: EventSource<K, M, APP>,
    EH: IsEventHandler<K, M, APP>,
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    APP: PartialEq + Eq + PartialOrd + Ord + Clone,
  > EventWatcher<ES, EH, K, M, APP>
{
  pub fn new(event_source: ES, event_handler: EH) -> Self {
    Self {
      event_source: event_source,
      event_handler: event_handler,
      _raw_key_type: std::marker::PhantomData,
      _raw_modifier_type: std::marker::PhantomData,
      _raw_app_identifier_type: std::marker::PhantomData,
    }
  }

  pub fn watch(&mut self) {
    self.event_source.remap_keys().unwrap();

    while let Some(event) = self.event_source.next() {
      match event {
        Event::KeyPressed(key_input) => self.event_handler.key_press(key_input),
        Event::KeyReleased(key_input) => self.event_handler.key_release(key_input),
        Event::ApplicationChange(application) => self.event_handler.change_application(application),
      }
    }
  }
}

#[derive(Clone)]
pub struct Remap<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
> {
  pub from: KeyInput<K, M>,
  pub to: KeyInput<K, M>,
}

pub trait Action {
  fn call(&self);
}

#[derive(Clone)]
pub struct ExecAction<
  A: Action + Clone,
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
> {
  pub from: KeyInput<K, M>,
  pub action: Box<A>,
}

pub trait KeyPresser<
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
>
{
  fn press(&self, key_input: KeyInput<K, M>);
  fn release(&self, key_input: KeyInput<K, M>);
}

pub struct EventHandler<
  KP: KeyPresser<K, M>,
  A: Action + Clone,
  K: PartialEq + Eq + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
> {
  pub current_application: Application<APP>,
  pub global_remaps: Vec<Remap<K, M>>,
  pub global_exec_actions: Vec<ExecAction<A, K, M>>,
  pub remaps_for_application: BTreeMap<Application<APP>, Vec<Remap<K, M>>>,
  pub exec_actions_for_application: BTreeMap<Application<APP>, Vec<ExecAction<A, K, M>>>,
  pub key_presser: KP,
}

impl<
    KP: KeyPresser<K, M>,
    A: Action + Clone,
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    APP: PartialEq + Eq + PartialOrd + Ord + Clone + std::fmt::Debug,
  > IsEventHandler<K, M, APP> for EventHandler<KP, A, K, M, APP>
{
  fn change_application(&mut self, application: Application<APP>) {
    println!(
      "before: {:?}, next: {:?}",
      self.current_application.name, application.name
    );
    self.current_application = application;
  }

  fn key_press(&self, key_input: KeyInput<K, M>) {
    for remap in self.current_remaps() {
      match remap.from.match_to(&key_input) {
        Matching::Unmatched => {}
        Matching::Remain(modifiers) => {
          let remapped_key_input = remap.to.merge_modifiers(&modifiers);
          println!("{:?}", remapped_key_input);
          self.key_presser.press(remapped_key_input);
          return;
        }
      }
    }

    for exec_action in self.current_exec_actions() {
      match exec_action.from.match_to(&key_input) {
        Matching::Unmatched => {}
        Matching::Remain(modifiers) => {
          if modifiers.is_empty() {
            // exact matchの場合のみactionを実行する
            exec_action.action.call();
            return;
          }
        }
      }
    }

    self.key_presser.press(key_input)
  }

  fn key_release(&self, key_input: KeyInput<K, M>) {
    for remap in self.current_remaps() {
      match remap.from.match_to(&key_input) {
        Matching::Unmatched => {}
        Matching::Remain(modifiers) => {
          let remapped_key_input = remap.to.merge_modifiers(&modifiers);
          println!("{:?}", remapped_key_input);
          self.key_presser.release(remapped_key_input);
          return;
        }
      }
    }

    for exec_action in self.current_exec_actions() {
      match exec_action.from.match_to(&key_input) {
        Matching::Unmatched => {}
        Matching::Remain(modifiers) => {
          if modifiers.is_empty() {
            // actionの場合はreleaseを発行する必要がないのでなにもせずに終了する
            // ここでreturnしないとreleaseが発行されてしまうのでreturnする
            return;
          }
        }
      }
    }

    self.key_presser.release(key_input)
  }
}

impl<
    KP: KeyPresser<K, M>,
    A: Action + Clone,
    K: PartialEq + Eq + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    APP: PartialEq + Eq + PartialOrd + Ord + Clone,
  > EventHandler<KP, A, K, M, APP>
{
  fn current_remaps(&self) -> Vec<Remap<K, M>> {
    vec![
      self.global_remaps.clone(),
      self
        .remaps_for_application
        .get(&self.current_application)
        .unwrap_or(&vec![])
        .clone(),
    ]
    .concat()
  }

  fn current_exec_actions(&self) -> Vec<ExecAction<A, K, M>> {
    vec![
      self.global_exec_actions.clone(),
      self
        .exec_actions_for_application
        .get(&self.current_application)
        .unwrap_or(&vec![])
        .clone(),
    ]
    .concat()
  }
}
