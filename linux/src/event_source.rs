use super::*;
use mapper::IsEventSource;
use std::ffi::CString;
use x11::xlib;

pub struct XEventSource {
  display: XDisplay,
}

impl IsEventSource<XKeySymbol, XModifier, XAppIdentifier> for XEventSource {
  fn ungrab_keys(&self) {
    unsafe {
      xlib::XUngrabKey(
        self.display,
        xlib::AnyKey,
        xlib::AnyModifier,
        xlib::XDefaultRootWindow(self.display),
      );
    }
  }

  fn grab_keys(&self, key_inputs: Vec<KeyInput>) {
    let display = self.display;
    unsafe {
      for key_input in key_inputs {
        let key = key_input.key().raw_value();
        let modifiers = key_input
          .modifiers()
          .to_vec()
          .into_iter()
          .fold(0, |sum, modifier| sum | modifier.raw_value());
        xlib::XGrabKey(
          display,
          xlib::XKeysymToKeycode(display, key) as i32,
          modifiers,
          xlib::XDefaultRootWindow(display),
          xlib::True,
          xlib::GrabModeAsync,
          xlib::GrabModeAsync,
        );
      }
      xlib::XSelectInput(
        display,
        xlib::XDefaultRootWindow(display),
        xlib::KeyPressMask | xlib::KeyReleaseMask | xlib::PropertyChangeMask,
      );
    }
  }

  fn next(&self) -> Option<Event> {
    let mut event: xlib::XEvent = xlib::XEvent { type_: 0 };

    loop {
      unsafe {
        xlib::XNextEvent(self.display, &mut event);
        match event {
          xlib::XEvent {
            type_: xlib::KeyPress,
          } => {
            let x_key_sym = xlib::XKeycodeToKeysym(self.display, event.key.keycode as u8, 0);
            let key = Key::new(x_key_sym);

            let modifier_bitmap: u32 = event.key.state;
            let mut modifiers = vec![];
            for i in 0..=31 {
              let mask = 1 << i;
              if (modifier_bitmap & mask) > 0 {
                modifiers.push(Modifier::new(mask))
              }
            }

            return Some(Event::KeyPressed {
              key_input: KeyInput::new(key, Modifiers::new(modifiers)),
            });
          }
          xlib::XEvent {
            type_: xlib::KeyRelease,
          } => {
            let x_key_sym = xlib::XKeycodeToKeysym(self.display, event.key.keycode as u8, 0);
            let key = Key::new(x_key_sym);

            let modifier_bitmap: u32 = event.key.state;
            let mut modifiers = vec![];
            for i in 0..=31 {
              let mask = 1 << i;
              if (modifier_bitmap & mask) > 0 {
                modifiers.push(Modifier::new(mask))
              }
            }

            return Some(Event::KeyReleased {
              key_input: KeyInput::new(key, Modifiers::new(modifiers)),
            });
          }
          xlib::XEvent {
            type_: xlib::PropertyNotify,
          } => {
            let application = self.fetch_focused_application();
            return Some(Event::ApplicationChanged {
              next_application: application,
            });
          }
          _ => {}
        }
      }
    }
  }
}

impl XEventSource {
  pub fn new(display: XDisplay) -> Self {
    Self { display }
  }

  fn fetch_focused_application(&self) -> Option<Application> {
    unsafe {
      let mut focused_window = 0;
      let mut focus_state = 0;
      xlib::XGetInputFocus(self.display, &mut focused_window, &mut focus_state);

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

      // WM_CLASSがとれるWindowsを引き当てるまで親方向にWindow treeを遡る
      // 引き当てたら、x_text_property.valueにその値が入っているはず
      let mut target_window: xlib::Window = focused_window;
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
        if parent == 0 {
          // root windowのparentは0になる。0にたいしてXGetTextProperyをすると死ぬのでここで終了する
          return None;
        }
        target_window = parent;
      }

      if x_text_property.nitems > 0 && !x_text_property.value.is_null() {
        if x_text_property.encoding == xlib::XA_STRING {
          Some(Application::new(
            CString::from_raw(x_text_property.value as *mut i8)
              .into_string()
              .unwrap(),
          ))
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
          Some(Application::new(name))
        }
      } else {
        None
      }
    }
  }
}
