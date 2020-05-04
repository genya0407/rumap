#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Application<AppIdentifier: PartialEq + Eq + PartialOrd + Ord + Clone> {
  identifier: AppIdentifier,
}

impl<AppIdentifier: PartialEq + Eq + PartialOrd + Ord + Clone> Application<AppIdentifier> {
  pub fn new(identifier: AppIdentifier) -> Self {
    Self {
      identifier: identifier,
    }
  }
}
