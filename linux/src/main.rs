#[macro_use]
extern crate log;

use mapper::config::IsParser;

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
  let config: mapper::config::Config = serde_json::from_reader(std::fs::File::open(config_fname)?)?;
  let parser = linux::config::XParser::build(&linux::config::XIntoDomain);
  let key_bind_for_focus = parser.build_keybind_for_focus(config.clone())?;
  let possible_keyinput_finder = parser.build_possible_keyinput_finder(config.clone())?;
  let display = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
  let event_source = linux::XEventSource::new(display);
  let key_handler = linux::XKeyHandler::new(display);
  let mut state = linux::State::new(
    key_bind_for_focus,
    possible_keyinput_finder,
    event_source,
    key_handler,
    linux::ShellCommandExecutor,
  );
  state.run();
  Ok(())
}
