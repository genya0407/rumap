use x11::xlib;

pub trait HasDisplay {
  fn display(&self) -> *mut xlib::Display;
}
