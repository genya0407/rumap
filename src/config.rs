use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Application(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyInput(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Action {
  Key(KeyInput),
  Command { execute: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Remap {
  pub from: KeyInput,
  pub to: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub remaps: Vec<Remap>,
  pub remaps_for_application: BTreeMap<Application, Vec<Remap>>,
}
