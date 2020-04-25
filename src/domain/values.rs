use std::collections::BTreeSet;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Application<A: PartialOrd + Ord + PartialEq + Eq> {
  pub name: A,
}

pub enum Matching<M> {
  Unmatched,
  Remain(Modifiers<M>),
}

#[derive(Clone)]
pub struct KeyInput<K: PartialEq + Eq + Clone, M: PartialOrd + Ord + Clone> {
  key: Key<K>,
  modifiers: Modifiers<M>,
}

impl<K: PartialEq + Eq + Clone, M: PartialOrd + Ord + Clone> KeyInput<K, M> {
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

#[derive(PartialEq, Eq, Clone)]
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

#[derive(Clone)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
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
