mod event_finished_error;

use self::event_finished_error::*;
use crate::{
  Action, Application, Event, Focus, IsEventSource, IsKeyHandler, IsShellCommandExecutor, KeyInput,
  PossibleKeyinputFinder, XAppIdentifier, XExecution, XKeySymbol, XModifier,
};
use mapper::IsKeyBindForFocus;

pub trait IsState {
  fn run(&mut self);
}

pub struct State<
  KBFF: IsKeyBindForFocus<XAppIdentifier, XKeySymbol, XModifier, XExecution>,
  ES: IsEventSource,
  KH: IsKeyHandler,
  SCE: IsShellCommandExecutor,
> {
  pub application: Option<Application>,
  shell_command_executor: SCE,
  event_source: ES,
  key_handler: KH,
  key_bind_for_focus: KBFF,
  possible_keyinput_finder: PossibleKeyinputFinder,
}

impl<
    KBFF: IsKeyBindForFocus<XAppIdentifier, XKeySymbol, XModifier, XExecution>,
    ES: IsEventSource,
    KH: IsKeyHandler,
    SCE: IsShellCommandExecutor,
  > State<KBFF, ES, KH, SCE>
{
  pub fn run(&mut self) {
    self.event_source.grab_keys(self.watch_target_key_inputs());

    loop {
      match self.event_source.next() {
        Some(Event::ApplicationChanged { next_application }) => {
          self.application = next_application;
          self.event_source.ungrab_keys();
          self.event_source.grab_keys(self.watch_target_key_inputs());
        }
        Some(Event::KeyPressed { key_input }) => {
          if let Some(action) = self
            .key_bind_for_focus
            .pressed(self.focus(), key_input.clone())
          {
            match action {
              Action::Key {
                key_input: bound_key_input,
              } => self.key_handler.press_key(bound_key_input),
              Action::Execution {
                execution: XExecution::ShellCommand(command),
              } => self.shell_command_executor.execute(command),
            }
          } else {
            self.key_handler.press_key(key_input)
          }
        }
        Some(Event::KeyReleased { key_input }) => {
          if let Some(action) = self
            .key_bind_for_focus
            .released(self.focus(), key_input.clone())
          {
            match action {
              Action::Key {
                key_input: bound_key_input,
              } => self.key_handler.release_key(bound_key_input),
              Action::Execution {
                execution: XExecution::ShellCommand(command),
              } => self.shell_command_executor.execute(command),
            }
          } else {
            self.key_handler.release_key(key_input)
          }
        }
        None => return,
      }
    }
  }
}

impl<
    KBFF: IsKeyBindForFocus<XAppIdentifier, XKeySymbol, XModifier, XExecution>,
    ES: IsEventSource,
    KH: IsKeyHandler,
    SCE: IsShellCommandExecutor,
  > State<KBFF, ES, KH, SCE>
{
  pub fn new(
    key_bind_for_focus: KBFF,
    possible_keyinput_finder: PossibleKeyinputFinder,
    event_source: ES,
    key_handler: KH,
    shell_command_executor: SCE,
  ) -> Self {
    Self {
      application: None,
      event_source: event_source,
      key_handler: key_handler,
      key_bind_for_focus: key_bind_for_focus,
      possible_keyinput_finder: possible_keyinput_finder,
      shell_command_executor: shell_command_executor,
    }
  }

  fn watch_target_key_inputs(&self) -> Vec<KeyInput> {
    let keyinputs = self.possible_keyinput_finder.find(self.focus());
    log::trace!("watch_target_key_inputs: {:?}", keyinputs);
    keyinputs
  }

  fn focus(&self) -> Focus {
    let f = match self.application.clone() {
      Some(app) => Focus::Focused { application: app },
      None => Focus::NoFocus,
    };
    log::trace!("{:?}", f);
    f
  }
}
