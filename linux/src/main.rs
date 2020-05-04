#[macro_use]
extern crate log;

use linux::{
  Action, Application, Event, Focus, IsEventSource, IsKeyHandler, KeyInput, PossibleKeyinputFinder,
  XAppIdentifier, XExecution, XKeySymbol, XModifier,
};
use mapper::config::IsParser;
use mapper::IsKeyBindForFocus;
use x11::xlib;

#[derive(Debug)]
struct EventFinishedError;

impl std::fmt::Display for EventFinishedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "EventFinished")
  }
}

impl std::error::Error for EventFinishedError {}

struct State<'a> {
  pub display: *mut xlib::Display,
  pub application: Option<Application>,
  pub key_bind_for_focus:
    mapper::KeyBindForFocus<'a, XAppIdentifier, XKeySymbol, XModifier, XExecution>,
  pub possible_keyinput_finder: PossibleKeyinputFinder,
}

impl<'a> State<'a> {
  pub fn new(config: mapper::config::Config) -> Result<Self, mapper::config::InvalidConfigError> {
    trace!("{:?}", config);
    let display = unsafe { xlib::XOpenDisplay(std::ptr::null()) };
    let parser = linux::config::XParser::build(&linux::config::XIntoDomain);
    let key_bind_for_focus = parser.build_keybind_for_focus(config.clone())?;
    let possible_keyinput_finder = parser.build_possible_keyinput_finder(config.clone())?;
    trace!("{:?}", possible_keyinput_finder);

    Ok(Self {
      display: display,
      application: None,
      key_bind_for_focus: key_bind_for_focus,
      possible_keyinput_finder: possible_keyinput_finder,
    })
  }

  pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.grab_keys(self.watch_target_key_inputs());

    loop {
      match self.next() {
        Some(Event::ApplicationChanged { next_application }) => {
          self.application = next_application;
          self.ungrab_keys();
          self.grab_keys(self.watch_target_key_inputs());
        }
        Some(Event::KeyPressed { key_input }) => {
          if let Some(action) = self
            .key_bind_for_focus
            .pressed(self.focus(), key_input.clone())
          {
            match action {
              Action::Key {
                key_input: bound_key_input,
              } => self.press_key(bound_key_input),
              Action::Execution {
                execution: XExecution::ShellCommand(command),
              } => self.exec_shell_command(command),
            }
          } else {
            self.press_key(key_input)
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
              } => self.release_key(bound_key_input),
              Action::Execution {
                execution: XExecution::ShellCommand(command),
              } => self.exec_shell_command(command),
            }
          } else {
            self.release_key(key_input)
          }
        }
        None => return Err(Box::new(EventFinishedError)),
      }
    }
  }

  fn watch_target_key_inputs(&self) -> Vec<KeyInput> {
    let keyinputs = self.possible_keyinput_finder.find(self.focus());
    trace!("watch_target_key_inputs: {:?}", keyinputs);
    keyinputs
  }

  fn focus(&self) -> Focus {
    let f = match self.application.clone() {
      Some(app) => Focus::Focused { application: app },
      None => Focus::NoFocus,
    };
    trace!("{:?}", f);
    f
  }

  fn exec_shell_command(&self, _command: String) {
    // FIXME
  }
}

impl<'a> linux::HasDisplay for State<'a> {
  fn display(&self) -> *mut xlib::Display {
    self.display
  }
}

impl<'a> linux::EventSource for State<'a> {}

impl<'a> linux::KeyHandler for State<'a> {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  use flexi_logger::Logger;

  Logger::with_env()
    .format(flexi_logger::opt_format)
    .start()
    .unwrap();

  trace!("start");

  let args = std::env::args().collect::<Vec<_>>();
  let config_fname = args.get(1).ok_or(std::io::Error::new(
    std::io::ErrorKind::Other,
    "config file not specified.",
  ))?;
  let config = serde_json::from_reader(std::fs::File::open(config_fname)?)?;
  let mut state = State::new(config)?;
  state.run()
}
