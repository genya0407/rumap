use std::ffi::CString;
use std::rc::Rc;
use std::sync::Mutex;
use x11::xlib;

use super::domain;

pub mod keysyms;

pub type XKeySymbol = u64;
pub type XModifier = u32;
pub type XAppIdentifier = String;

#[derive(Clone)]
pub enum XAction {
  Command(String),
}

impl domain::Action for XAction {
  fn call(&self) {}
}

pub struct XState {
  pub display: *mut xlib::Display,
  pub window: xlib::Window,
}

impl XState {
  pub fn fetch_current_application(&self) -> domain::Application<XAppIdentifier> {
    unsafe {
      let class_atom = xlib::XInternAtom(
        self.display,
        CString::new("WM_CLASS").unwrap().as_ptr(),
        xlib::True,
      );

      let mut x_text_property = xlib::XTextProperty {
        encoding: 0,
        nitems: 0,
        format: 0,
        value: &mut 0,
      };

      // XGetTextProperty can return 0 when xmonad's workspace is selected, while it usually returns 1.
      // This prevents SEGV in such a situation.
      // if xlib::XGetTextProperty(self.display, self.window, &mut x_text_property, class_atom) == 0 {
      //   return domain::Application {
      //     kind: String::from(""),
      //   };
      // }

      // WM_CLASSがとれるWindowsを引き当てるまで親方向にWindow treeを遡る
      // 引き当てたら、x_text_property.valueにその値が入っているはず
      let mut target_window: xlib::Window = self.window;
      loop {
        if xlib::XGetTextProperty(
          self.display,
          target_window,
          &mut x_text_property,
          class_atom,
        ) == 1
        {
          break;
        }

        let mut nchildren: u32 = 0;
        let mut root: xlib::Window = 0;
        let mut parent: xlib::Window = 0;
        let mut children: *mut xlib::Window = &mut 0;

        if xlib::XQueryTree(
          self.display,
          target_window,
          &mut root,
          &mut parent,
          &mut children,
          &mut nchildren,
        ) == 0
        {
          break;
        }
        if !children.is_null() {
          xlib::XFree(children as *mut std::ffi::c_void);
        }
        if parent != 0 {
          target_window = parent;
        } else {
          // root windowのparentは0になる。0にたいしてXGetTextProperyをすると死ぬのでここでデフォルト値を返す。
          return domain::Application {
            kind: String::from(""),
          };
        }
      }

      if x_text_property.nitems > 0 && !x_text_property.value.is_null() {
        if x_text_property.encoding == xlib::XA_STRING {
          domain::Application {
            kind: CString::from_raw(x_text_property.value as *mut i8)
              .into_string()
              .unwrap(),
          }
        } else {
          let mut char_list: *mut *mut i8 = std::ptr::null_mut();
          let mut count: i32 = 0;
          xlib::XmbTextPropertyToTextList(
            self.display,
            &x_text_property,
            &mut char_list,
            &mut count,
          );
          let name = if count > 0 && !(*char_list).is_null() {
            CString::from_raw(*char_list).into_string().unwrap()
          } else {
            String::from("")
          };
          xlib::XFreeStringList(char_list);
          domain::Application { kind: name }
        }
      } else {
        domain::Application {
          kind: String::from(""),
        }
      }
    }
  }
}

// Ctrl-Shift-j みたいな文字列をパースする
pub fn parse_key_input(
  key_input: crate::config::KeyInput,
) -> Result<domain::KeyInput<XKeySymbol, XModifier>, Box<dyn std::error::Error>> {
  let key_input = key_input.0;
  let mut key_names = key_input.split('-').collect::<Vec<&str>>();
  if key_names.len() == 0 {
    return Err("empty key".into());
  }
  // Modifier-Modifier-...-Key となっているのをパースする
  let key_name = key_names.pop().unwrap();
  let modifier_names = key_names;
  let keysym = crate::x::keysyms::KEYNAME_TO_KEYSYM
    .get(key_name)
    .cloned()
    .ok_or(std::io::Error::new(
      std::io::ErrorKind::Other,
      format!("Unexpected key: {}", key_name),
    ))?;
  let mut modifier_masks = vec![];
  for modifier_name in modifier_names {
    modifier_masks.push(
      crate::x::keysyms::MODIFIERNAME_TO_MASK
        .get(modifier_name)
        .cloned()
        .ok_or(std::io::Error::new(
          std::io::ErrorKind::Other,
          format!("Unexpected modifier: {}", modifier_name),
        ))?,
    );
  }
  Ok(domain::KeyInput::new(
    domain::Key::new(keysym),
    domain::Modifiers::new(
      modifier_masks
        .into_iter()
        .map(|mask| domain::Modifier::new(mask))
        .collect(),
    ),
  ))
}

pub struct XEventSource {
  state: Rc<Mutex<XState>>,
  watch_target_key_inputs: Vec<domain::KeyInput<XKeySymbol, XModifier>>,
}

impl XEventSource {
  pub fn build(
    config: crate::config::Config,
    state: Rc<Mutex<XState>>,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    // TODO: window毎にregisterするkeyをかえる
    let remaps = vec![
      config.remaps.clone(),
      config
        .remaps_for_application
        .values()
        .cloned()
        .collect::<Vec<_>>()
        .concat(),
    ]
    .concat();

    let mut watch_target_key_inputs: Vec<domain::KeyInput<XKeySymbol, XModifier>> = vec![];
    for remap in remaps {
      use itertools::Itertools;

      let from = parse_key_input(remap.from)?;
      let modifiers = from.modifiers().to_vec();
      for i in 0..=modifiers.len() {
        watch_target_key_inputs.extend(modifiers.clone().into_iter().combinations(i).map(
          |combination| domain::KeyInput::new(from.key(), domain::Modifiers::new(combination)),
        ));
      }
    }
    Ok(Self {
      state: state,
      watch_target_key_inputs: watch_target_key_inputs,
    })
  }
}

impl domain::EventSource<XKeySymbol, XModifier, XAppIdentifier> for XEventSource {
  fn initialize_register_state(&self) -> Result<(), Box<dyn std::error::Error>> {
    let display = { self.state.lock().unwrap().display };
    unsafe {
      xlib::XUngrabKey(
        display,
        xlib::AnyKey,
        xlib::AnyModifier,
        xlib::XDefaultRootWindow(display),
      );
      Ok(())
    }
  }

  fn register_key(
    &self,
    key_input: domain::KeyInput<XKeySymbol, XModifier>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let display = { self.state.lock().unwrap().display };
    let key = key_input.key().raw_value();
    let modifiers = key_input
      .modifiers()
      .to_vec()
      .into_iter()
      .fold(0, |sum, modifier| sum | modifier.raw_value());
    unsafe {
      // TODO ungrab keys before grab keys
      xlib::XGrabKey(
        display,
        xlib::XKeysymToKeycode(display, key) as i32,
        modifiers,
        xlib::XDefaultRootWindow(display),
        xlib::True,
        xlib::GrabModeAsync,
        xlib::GrabModeAsync,
      );
      xlib::XSelectInput(
        display,
        xlib::XDefaultRootWindow(display),
        xlib::KeyPressMask | xlib::PropertyChangeMask,
      );
    }
    Ok(())
  }

  fn next(&self) -> Option<domain::Event<XKeySymbol, XModifier, XAppIdentifier>> {
    let mut event: xlib::XEvent = xlib::XEvent { type_: 0 };

    loop {
      unsafe {
        let display = { self.state.lock().unwrap().display };
        xlib::XNextEvent(display, &mut event);
        match event {
          xlib::XEvent {
            type_: xlib::KeyPress,
          } => {
            let x_key_sym = xlib::XKeycodeToKeysym(display, event.key.keycode as u8, 0);
            let key = domain::Key::new(x_key_sym);

            let modifier_bitmap: u32 = event.key.state;
            let mut modifiers = vec![];
            for i in 0..=31 {
              let mask = 1 << i;
              if (modifier_bitmap & mask) > 0 {
                modifiers.push(domain::Modifier::new(mask))
              }
            }

            return Some(domain::Event::KeyPressed(domain::KeyInput::new(
              key,
              domain::Modifiers::new(modifiers),
            )));
          }
          xlib::XEvent {
            type_: xlib::KeyRelease,
          } => {
            let x_key_sym = xlib::XKeycodeToKeysym(display, event.key.keycode as u8, 0);
            let key = domain::Key::new(x_key_sym);

            let modifier_bitmap: u32 = event.key.state;
            let mut modifiers = vec![];
            for i in 0..=31 {
              let mask = 1 << i;
              if (modifier_bitmap & mask) > 0 {
                modifiers.push(domain::Modifier::new(mask))
              }
            }

            return Some(domain::Event::KeyReleased(domain::KeyInput::new(
              key,
              domain::Modifiers::new(modifiers),
            )));
          }
          xlib::XEvent {
            type_: xlib::PropertyNotify,
          } => {
            let current_application = {
              let mut state = self.state.lock().unwrap();
              let mut focused_window = 0;
              let mut focus_state = 0;
              xlib::XGetInputFocus(display, &mut focused_window, &mut focus_state);
              state.window = focused_window;
              state.fetch_current_application()
            };
            return Some(domain::Event::ApplicationChange(current_application));
          }
          _ => {}
        }
      }
    }
  }

  fn watch_target_key_inputs(&self) -> Vec<domain::KeyInput<XKeySymbol, XModifier>> {
    self.watch_target_key_inputs.clone()
  }
}

pub struct XKeyPresser {
  pub state: Rc<Mutex<XState>>,
}

impl XKeyPresser {
  fn key_event(&self, key_input: domain::KeyInput<XKeySymbol, XModifier>, evt_type: i32) {
    unsafe {
      let display = { self.state.lock().unwrap().display };

      let mut focused_window = 0;
      let mut focus_state = 0;
      xlib::XGetInputFocus(display, &mut focused_window, &mut focus_state);

      let modifier_bits = key_input
        .modifiers()
        .to_vec()
        .into_iter()
        .fold(0, |bits, modifier| bits | modifier.raw_value());
      let key_event = xlib::XKeyEvent {
        type_: evt_type,
        serial: 0,
        send_event: xlib::True,
        display: display,
        window: focused_window,
        root: xlib::XDefaultRootWindow(display),
        subwindow: 0,
        time: xlib::CurrentTime,
        x: 1,
        y: 1,
        x_root: 1,
        y_root: 1,
        state: modifier_bits,
        keycode: xlib::XKeysymToKeycode(display, key_input.key().raw_value()) as u32,
        same_screen: xlib::True,
      };
      let event = xlib::XEvent { key: key_event };

      xlib::XSendEvent(
        display,
        focused_window,
        xlib::True,
        xlib::KeyPressMask,
        &mut (event as xlib::XEvent),
      );
    }
  }
}

impl domain::KeyPresser<XKeySymbol, XModifier> for XKeyPresser {
  fn press(&self, key_input: domain::KeyInput<XKeySymbol, XModifier>) {
    self.key_event(key_input, xlib::KeyPress);
  }

  fn release(&self, key_input: domain::KeyInput<XKeySymbol, XModifier>) {
    self.key_event(key_input, xlib::KeyRelease);
  }
}
