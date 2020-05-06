use std::rc::Rc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MockEventSource {
  pub event_sequence: Rc<Mutex<Vec<linux::Event>>>,
  pub ungrabbed_count: Rc<Mutex<usize>>,
  pub grabbed_keys: Rc<Mutex<Vec<Vec<linux::KeyInput>>>>,
}

impl MockEventSource {
  pub fn new(mut event_sequence: Vec<linux::Event>) -> Self {
    event_sequence.reverse();
    Self {
      event_sequence: Rc::new(Mutex::new(event_sequence)),
      ungrabbed_count: Rc::new(Mutex::new(0)),
      grabbed_keys: Rc::new(Mutex::new(vec![])),
    }
  }
}

impl linux::IsEventSource for MockEventSource {
  fn ungrab_keys(&self) {
    let mut count = self.ungrabbed_count.lock().unwrap();
    *count += 1;
  }

  fn grab_keys(&self, key_inputs: Vec<linux::KeyInput>) {
    self.grabbed_keys.lock().unwrap().push(key_inputs)
  }

  fn next(&self) -> Option<linux::Event> {
    self.event_sequence.lock().unwrap().pop()
  }
}
