pub mod config;

mod event_source;
mod execution_key_bind;
mod key_handler;
mod shell_command_executor;
mod state;

pub use event_source::*;
pub use execution_key_bind::*;
pub use key_handler::*;
pub use shell_command_executor::*;
pub use state::*;

pub type XAppIdentifier = String;
pub type XKeySymbol = u64;
pub type XModifier = u32;
#[derive(Debug, Clone)]
pub enum XExecution {
  ShellCommand(String),
}

pub type Application = mapper::Application<XAppIdentifier>;
pub type KeyInput = mapper::KeyInput<XKeySymbol, XModifier>;
pub type Key = mapper::Key<XKeySymbol>;
pub type Modifier = mapper::Modifier<XModifier>;
pub type Modifiers = mapper::Modifiers<XModifier>;
pub type Focus = mapper::Focus<XAppIdentifier>;
pub type Action = mapper::Action<XKeySymbol, XModifier, XExecution>;
pub type PossibleKeyinputFinder =
  mapper::PossibleKeyinputFinder<XAppIdentifier, XKeySymbol, XModifier>;

pub type XDisplay = *mut x11::xlib::Display;
