#[macro_use]
extern crate log;

use mapper::config::IsParser;

#[macro_use]
extern crate clap;
use clap::App;
use std::io::Write;

use std::process::Command;

fn fetch_config() -> Option<mapper::config::Config> {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  if let Some(xremap_config_fname) = matches.value_of("xremap_config") {
    let mut converter_rb = tempfile::NamedTempFile::new().ok()?;
    let converter_rb_source = include_str!("convert.rb");
    converter_rb
      .write_all(converter_rb_source.as_bytes())
      .ok()?;
    let output = Command::new("ruby")
      .args(vec![converter_rb.path().to_str()?, xremap_config_fname])
      .output()
      .ok()?;
    return serde_json::from_reader::<&[u8], mapper::config::Config>(output.stdout.as_ref()).ok();
  }

  if let Some(config_fname) = matches.value_of("config") {
    return serde_json::from_reader(std::fs::File::open(config_fname).ok()?).ok();
  }

  None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  use flexi_logger::Logger;

  Logger::with_env()
    .format(flexi_logger::opt_format)
    .start()
    .unwrap();

  trace!("start");

  let config = fetch_config().unwrap();
  let parser = linux::config::XParser::build(&linux::config::XIntoDomain);
  let key_bind_for_focus = parser.build_keybind_for_focus(config.clone())?;
  let possible_keyinput_finder = parser.build_possible_keyinput_finder(config.clone())?;
  let display = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
  let event_source = linux::XEventSource::new(display);
  let key_handler = linux::XKeyHandler::new(display);
  let mut state = mapper::State::new(
    key_bind_for_focus,
    possible_keyinput_finder,
    event_source,
    key_handler,
    linux::ShellCommandExecutor,
  );
  state.run();
  Ok(())
}
