use super::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PossibleKeyinputFinder<
  A: PartialEq + Eq + PartialOrd + Ord + Clone,
  K: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
> {
  cache: BTreeMap<Focus<A>, Vec<KeyInput<K, M>>>,
}

impl<
    A: PartialEq + Eq + PartialOrd + Ord + Clone,
    K: PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
  > PossibleKeyinputFinder<A, K, M>
{
  pub fn new(cache: BTreeMap<Focus<A>, Vec<KeyInput<K, M>>>) -> Self {
    Self { cache: cache }
  }

  // focusedであってもnofocusの結果をmergeして返す
  // FIXME ほんとはinappがglobalを上書きするようにしたい
  pub fn find(&self, focus: Focus<A>) -> Vec<KeyInput<K, M>> {
    let keyinputs_for_nofocus = self.cache.get(&Focus::NoFocus).cloned().unwrap_or_default();
    match focus {
      Focus::Focused { application } => {
        let mut all = vec![
          keyinputs_for_nofocus,
          self
            .cache
            .get(&Focus::Focused { application })
            .cloned()
            .unwrap_or_default(),
        ]
        .concat();
        all.sort();
        all.dedup();
        all
      }
      Focus::NoFocus => keyinputs_for_nofocus,
    }
  }
}
