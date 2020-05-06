use crate::{
  Action, Application, Event, Focus, IsEventSource, IsKeyHandler, KeyInput, PossibleKeyinputFinder,
  XAppIdentifier, XExecution, XKeySymbol, XModifier,
};
use mapper::IsKeyBindForFocus;

#[derive(Debug)]
pub struct EventFinishedError;

impl std::fmt::Display for EventFinishedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "EventFinished")
  }
}

impl std::error::Error for EventFinishedError {}

pub trait IsState {
  fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct State<'a, ES: IsEventSource, KH: IsKeyHandler> {
  pub application: Option<Application>,
  pub event_source: ES,
  pub key_handler: KH,
  pub key_bind_for_focus:
    mapper::KeyBindForFocus<'a, XAppIdentifier, XKeySymbol, XModifier, XExecution>,
  pub possible_keyinput_finder: PossibleKeyinputFinder,
}

impl<'a, ES: IsEventSource, KH: IsKeyHandler> State<'a, ES, KH> {
  pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
              } => self.exec_shell_command(command),
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
              } => self.exec_shell_command(command),
            }
          } else {
            self.key_handler.release_key(key_input)
          }
        }
        None => return Err(Box::new(EventFinishedError)),
      }
    }
  }
}

impl<'a, ES: IsEventSource, KH: IsKeyHandler> State<'a, ES, KH> {
  pub fn new(
    key_bind_for_focus: mapper::KeyBindForFocus<
      'a,
      XAppIdentifier,
      XKeySymbol,
      XModifier,
      XExecution,
    >,
    possible_keyinput_finder: PossibleKeyinputFinder,
    event_source: ES,
    key_handler: KH,
  ) -> Result<Self, mapper::config::InvalidConfigError> {
    Ok(Self {
      application: None,
      event_source: event_source,
      key_handler: key_handler,
      key_bind_for_focus: key_bind_for_focus,
      possible_keyinput_finder: possible_keyinput_finder,
    })
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

  fn exec_shell_command(&self, _command: String) {
    // FIXME: 別のtraitに切り出してinjectする
  }
}
