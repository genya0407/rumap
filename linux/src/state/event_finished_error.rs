#[derive(Debug)]
pub struct EventFinishedError;

impl std::fmt::Display for EventFinishedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "EventFinished")
  }
}

impl std::error::Error for EventFinishedError {}
