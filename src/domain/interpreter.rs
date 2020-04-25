use super::event::*;
use super::values::*;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Remap<K: PartialEq + Eq + Clone, M: PartialOrd + Ord + Clone> {
  pub from: KeyInput<K, M>,
  pub to: KeyInput<K, M>,
}

pub trait Action {
  fn call(&self);
}

#[derive(Clone)]
pub struct ExecAction<A: Action + Clone, K: PartialEq + Eq + Clone, M: PartialOrd + Ord + Clone> {
  pub from: KeyInput<K, M>,
  pub action: Box<A>,
}

pub trait KeyPresser<K: PartialEq + Eq + Clone, M: PartialOrd + Ord + Clone> {
  fn press(&self, key_input: KeyInput<K, M>);
}

pub struct Interpreter<
  KP: KeyPresser<K, M>,
  A: Action + Clone,
  K: PartialEq + Eq + Clone,
  M: PartialOrd + Ord + Clone,
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
    K: PartialEq + Eq + Clone,
    M: PartialOrd + Ord + Clone,
    APP: PartialEq + Eq + PartialOrd + Ord + Clone,
  > EventHandler<K, M, APP> for Interpreter<KP, A, K, M, APP>
{
  fn change_application(&mut self, application: Application<APP>) {
    self.current_application = application;
  }

  fn key_press(&self, key_input: KeyInput<K, M>) {
    for remap in self.current_remaps() {
      match remap.from.match_to(&key_input) {
        Matching::Unmatched => {}
        Matching::Remain(modifiers) => {
          let remapped_key_input = remap.to.merge_modifiers(&modifiers);
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
}

impl<
    KP: KeyPresser<K, M>,
    A: Action + Clone,
    K: PartialEq + Eq + Clone,
    M: PartialOrd + Ord + Clone,
    APP: PartialEq + Eq + PartialOrd + Ord + Clone,
  > Interpreter<KP, A, K, M, APP>
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
