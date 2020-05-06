use super::KeyInput;
use super::XDisplay;
use x11::xlib;

pub trait IsKeyHandler {
  fn press_key(&self, key_input: KeyInput);
  fn release_key(&self, key_input: KeyInput);
}

pub struct XKeyHandler {
  display: XDisplay,
}

impl IsKeyHandler for XKeyHandler {
  fn press_key(&self, key_input: KeyInput) {
    self.key_event(self.display, key_input, xlib::KeyPress);
  }

  fn release_key(&self, key_input: KeyInput) {
    self.key_event(self.display, key_input, xlib::KeyRelease);
  }
}

impl XKeyHandler {
  pub fn new(display: XDisplay) -> Self {
    Self { display }
  }

  fn key_event(&self, display: *mut xlib::Display, key_input: KeyInput, evt_type: i32) {
    unsafe {
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
