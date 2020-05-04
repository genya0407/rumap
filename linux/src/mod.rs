// use std::rc::Rc;
// use std::sync::Mutex;
// use x11::xlib;

// use super::domain;

// mod config;
// mod event_source;
// mod keysyms;
// mod remapper;
// mod state;

// pub use state::*;

// pub type XKeySymbol = u64;
// pub type XModifier = u32;
// pub type KeyInput = domain::KeyInput<XKeySymbol, XModifier>;
// pub type Key = domain::Key<XKeySymbol>;
// pub type Modifier = domain::Modifier<XModifier>;
// pub type Application = domain::Application<String>;

// pub struct XEventSource {
//   state: Rc<Mutex<State>>,
//   watch_target_key_inputs: Vec<domain::KeyInput<XKeySymbol, XModifier>>,
// }

// impl XEventSource {
//   pub fn build(
//     config: crate::config::Config,
//     state: Rc<Mutex<State>>,
//   ) -> Result<Self, Box<dyn std::error::Error>> {
//     // TODO: window毎にregisterするkeyをかえる
//     let remaps = vec![
//       config.remaps.clone(),
//       config
//         .remaps_for_application
//         .values()
//         .cloned()
//         .collect::<Vec<_>>()
//         .concat(),
//     ]
//     .concat();

//     let mut watch_target_key_inputs: Vec<domain::KeyInput<XKeySymbol, XModifier>> = vec![];
//     for remap in remaps {
//       use itertools::Itertools;

//       let from = parse_key_input(remap.from)?;
//       let modifiers = from.modifiers().to_vec();
//       for i in 0..=modifiers.len() {
//         watch_target_key_inputs.extend(modifiers.clone().into_iter().combinations(i).map(
//           |combination| domain::KeyInput::new(from.key(), domain::Modifiers::new(combination)),
//         ));
//       }
//     }
//     Ok(Self {
//       state: state,
//       watch_target_key_inputs: watch_target_key_inputs,
//     })
//   }
// }

// pub struct XKeyPresser {
//   pub state: Rc<Mutex<State>>,
// }
