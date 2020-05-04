use super::Application;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Focus<AppIdentifier: PartialEq + Eq + PartialOrd + Ord + Clone> {
  Focused {
    application: Application<AppIdentifier>,
  },
  NoFocus,
}
