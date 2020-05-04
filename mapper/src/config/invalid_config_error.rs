#[derive(Debug)]
pub enum InvalidConfigError {
  EmptyKey,
  UnexpectedKey(String),
  UnexpectedModifier(String),
}

impl std::fmt::Display for InvalidConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::error::Error for InvalidConfigError {}
