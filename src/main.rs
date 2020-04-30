use std::collections::BTreeMap;
use std::env;
use std::ptr::null;
use std::rc::Rc;
use std::sync::Mutex;
use x11::xlib;

mod config;
mod domain;
mod x;

use x::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = env::args().collect();
    let config_json_path = argv.get(1).expect("specify config file name."); // TODO: cliコマンドとしての体をなす
    let config: config::Config = serde_json::from_str(&std::fs::read_to_string(config_json_path)?)?;

    unsafe {
        let display = xlib::XOpenDisplay(null());
        let window = xlib::XDefaultRootWindow(display);
        let state: Rc<Mutex<XState>> = Rc::new(Mutex::new(XState {
            display: display,
            window: window,
        }));
        let event_source = XEventSource::build(config.clone(), state.clone())?;
        let key_presser = XKeyPresser {
            state: state.clone(),
        };
        let current_application = {
            let state = state.lock().unwrap();
            state.fetch_current_application()
        };

        let global_remaps = config
            .remaps
            .clone()
            .into_iter()
            .filter_map(|remap| match remap.to {
                config::Action::Key(key_input) => Some(domain::interpreter::Remap {
                    from: x::parse_key_input(remap.from).unwrap(), // TODO unwrap
                    to: x::parse_key_input(key_input).unwrap(),    // TODO unwrap
                }),
                _ => None,
            })
            .collect();

        let mut remaps_for_application: BTreeMap<
            domain::values::Application<x::XAppIdentifier>,
            Vec<domain::interpreter::Remap<x::XKeySymbol, x::XModifier>>,
        > = BTreeMap::new();
        for (config_application, config_remaps) in config.remaps_for_application.clone().into_iter()
        {
            let application = domain::values::Application {
                name: config_application.0,
            };
            let mut domain_remaps = vec![];
            for config_remap in config_remaps.iter() {
                match config_remap.to.clone() {
                    config::Action::Key(key_input) => {
                        domain_remaps.push(domain::interpreter::Remap {
                            from: x::parse_key_input(config_remap.from.clone())?,
                            to: x::parse_key_input(key_input.clone())?,
                        })
                    }
                    _ => {}
                }
            }
            if domain_remaps.len() > 0 {
                remaps_for_application.insert(application, domain_remaps);
            }
        }

        let global_exec_actions = config
            .remaps
            .clone()
            .into_iter()
            .filter_map(|remap| match remap.to {
                config::Action::Command { execute: command } => {
                    Some(domain::interpreter::ExecAction {
                        from: x::parse_key_input(remap.from).unwrap(), // TODO unwrap
                        action: Box::new(x::XAction::Command(command)),
                    })
                }
                _ => None,
            })
            .collect();

        let mut exec_actions_for_application: BTreeMap<
            domain::values::Application<x::XAppIdentifier>,
            Vec<domain::interpreter::ExecAction<x::XAction, x::XKeySymbol, x::XModifier>>,
        > = BTreeMap::new();
        for (config_application, config_remaps) in config.remaps_for_application.clone().into_iter()
        {
            let application = domain::values::Application {
                name: config_application.0,
            };
            let mut domain_remaps = vec![];
            for config_remap in config_remaps.iter() {
                match config_remap.to.clone() {
                    config::Action::Command { execute: command } => {
                        domain_remaps.push(domain::interpreter::ExecAction {
                            from: x::parse_key_input(config_remap.from.clone())?,
                            action: Box::new(x::XAction::Command(command)),
                        })
                    }
                    _ => {}
                }
            }
            if domain_remaps.len() > 0 {
                exec_actions_for_application.insert(application, domain_remaps);
            }
        }

        let interpreter: domain::interpreter::Interpreter<
            XKeyPresser,
            XAction,
            XKeySymbol,
            XModifier,
            XAppIdentifier,
        > = domain::interpreter::Interpreter {
            current_application: current_application,
            global_remaps: global_remaps,
            global_exec_actions: global_exec_actions,
            remaps_for_application: remaps_for_application,
            exec_actions_for_application: exec_actions_for_application,
            key_presser: key_presser,
        };
        let mut event_watcher: domain::event::EventWatcher<
            XEventSource,
            domain::interpreter::Interpreter<
                XKeyPresser,
                XAction,
                XKeySymbol,
                XModifier,
                XAppIdentifier,
            >,
            XKeySymbol,
            XModifier,
            XAppIdentifier,
        > = domain::event::EventWatcher::new(event_source, interpreter);
        event_watcher.watch();
        Ok(())
    }
}
