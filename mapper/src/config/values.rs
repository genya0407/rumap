use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Application(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct KeyInput(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Key(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Modifier(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Execution(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Action {
  KeyInput {
    to: KeyInput,
    with: Option<Vec<Modifier>>,
  },
  Execution {
    execute: Execution,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Remaps(pub BTreeMap<KeyInput, Action>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub remap: Remaps,
  pub in_app: BTreeMap<Application, Remaps>,
}
