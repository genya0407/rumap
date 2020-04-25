use super::values::*;

pub enum Event<
  K: PartialEq + Eq + Clone,
  M: PartialOrd + Ord + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
> {
  KeyPressed(KeyInput<K, M>),
  ApplicationChange(Application<APP>),
}

pub trait EventSource<
  K: PartialEq + Eq + Clone,
  M: PartialOrd + Ord + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
>
{
  fn register_keys(&self) -> Result<(), Box<dyn std::error::Error>> {
    for key_input in self.watch_target_key_inputs() {
      self.register_key(key_input)?;
    }
    Ok(())
  }
  fn register_key(&self, key_input: KeyInput<K, M>) -> Result<(), Box<dyn std::error::Error>>;
  fn next(&self) -> Option<Event<K, M, APP>>;
  fn watch_target_key_inputs(&self) -> Vec<KeyInput<K, M>>;
}

pub trait EventHandler<
  K: PartialEq + Eq + Clone,
  M: PartialOrd + Ord + Clone,
  APP: PartialEq + Eq + PartialOrd + Ord + Clone,
>
{
  fn key_press(&self, key_input: KeyInput<K, M>);
  fn change_application(&mut self, application: Application<APP>);
}

pub struct EventWatcher<
  ES: EventSource<K, M, APP>,
  EH: EventHandler<K, M, APP>,
  K: PartialEq + Eq + Clone,
  M: PartialOrd + Ord + Clone,
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
    EH: EventHandler<K, M, APP>,
    K: PartialEq + Eq + Clone,
    M: PartialOrd + Ord + Clone,
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
    self.event_source.register_keys().unwrap();

    while let Some(event) = self.event_source.next() {
      match event {
        Event::KeyPressed(key_input) => self.event_handler.key_press(key_input),
        Event::ApplicationChange(application) => self.event_handler.change_application(application),
      }
    }
  }
}
