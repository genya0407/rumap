pub trait IsShellCommandExecutor {
  fn execute(&self, command: String);
}

pub struct ShellCommandExecutor;

impl IsShellCommandExecutor for ShellCommandExecutor {
  fn execute(&self, command: String) {
    std::process::Command::new("sh")
      .arg("-c")
      .arg(&command)
      .spawn()
      .expect(&format!("failed to start command: {}", command));
  }
}
