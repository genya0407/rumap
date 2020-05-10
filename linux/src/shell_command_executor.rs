use crate::*;
use mapper::IsShellCommandExecutor;

pub struct ShellCommandExecutor;

impl IsShellCommandExecutor<XExecution> for ShellCommandExecutor {
  fn execute(&self, command: XExecution) {
    match command {
      XExecution::ShellCommand(cmd) => {
        std::process::Command::new("sh")
          .arg("-c")
          .arg(&cmd)
          .spawn()
          .expect(&format!("failed to start command: {}", cmd));
      }
    }
  }
}
