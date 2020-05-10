pub trait IsShellCommandExecutor<C: std::fmt::Debug + Clone> {
  fn execute(&self, command: C);
}
