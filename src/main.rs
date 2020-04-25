use std::collections::BTreeMap;
use std::ffi::CString;
use std::ptr::null;
use std::rc::Rc;
use std::sync::Mutex;
use x11::xlib;

mod domain;

type XKeySymbol = u64;
type XModifier = u32;
type XAppIdentifier = String;

#[derive(Clone)]
enum XAction {
    Command(String),
}

impl domain::interpreter::Action for XAction {
    fn call(&self) {}
}

struct XState {
    pub display: *mut xlib::Display,
    pub window: xlib::Window,
}

impl XState {
    fn fetch_current_application(&self) -> domain::values::Application<XAppIdentifier> {
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
            if xlib::XGetTextProperty(self.display, self.window, &mut x_text_property, class_atom)
                == 0
            {
                return domain::values::Application {
                    name: String::from(""),
                };
            }

            // WM_CLASSがとれるWindowsを引き当てるまで親方向にWindow treeを遡る
            // 引き当てたら、x_text_property.valueにその値が入っているはず
            loop {
                if xlib::XGetTextProperty(
                    self.display,
                    self.window,
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
                    self.window,
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
            }

            if x_text_property.nitems > 0
                && !x_text_property.value.is_null()
                && x_text_property.encoding == xlib::XA_STRING
            {
                domain::values::Application {
                    name: CString::from_raw(x_text_property.value as *mut i8)
                        .into_string()
                        .unwrap(),
                }
            } else {
                domain::values::Application {
                    name: String::from(""),
                }
            }
        }
    }
}

struct XEventSource {
    pub state: Rc<Mutex<XState>>,
    pub watch_target_key_inputs: Vec<domain::values::KeyInput<XKeySymbol, XModifier>>,
}

impl domain::event::EventSource<XKeySymbol, XModifier, XAppIdentifier> for XEventSource {
    fn register_key(
        &self,
        key_input: domain::values::KeyInput<XKeySymbol, XModifier>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let state = self.state.lock().unwrap();
        let key = key_input.key().raw_value();
        let modifiers = key_input
            .modifiers()
            .to_vec()
            .into_iter()
            .fold(0, |sum, modifier| sum | modifier.raw_value());
        unsafe {
            xlib::XGrabKey(
                state.display,
                xlib::XKeysymToKeycode(state.display, key) as i32,
                modifiers,
                xlib::XDefaultRootWindow(state.display),
                xlib::True,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
            );
            xlib::XSelectInput(
                state.display,
                xlib::XDefaultRootWindow(state.display),
                xlib::KeyPressMask | xlib::PropertyChangeMask,
            );
        }
        Ok(())
    }

    fn next(&self) -> Option<domain::event::Event<XKeySymbol, XModifier, XAppIdentifier>> {
        let mut event: xlib::XEvent = xlib::XEvent { type_: 0 };

        loop {
            unsafe {
                let state = self.state.lock().unwrap();
                xlib::XNextEvent(state.display, &mut event);
                match event {
                    xlib::XEvent {
                        type_: xlib::KeyPress,
                    } => {
                        let x_key_sym =
                            xlib::XKeycodeToKeysym(state.display, event.key.keycode as u8, 0);
                        let key = domain::values::Key::new(x_key_sym);

                        let modifier_bitmap: u32 = event.key.state;
                        let mut modifiers = vec![];
                        for i in 0..=31 {
                            let mask = 1 << i;
                            if (modifier_bitmap & mask) > 0 {
                                modifiers.push(domain::values::Modifier::new(mask))
                            }
                        }

                        return Some(domain::event::Event::KeyPressed(
                            domain::values::KeyInput::new(
                                key,
                                domain::values::Modifiers::new(modifiers),
                            ),
                        ));
                    }
                    xlib::XEvent {
                        type_: xlib::PropertyNotify,
                    } => {
                        let mut state = self.state.lock().unwrap();
                        state.window = event.property.window;
                        let current_application = state.fetch_current_application();
                        return Some(domain::event::Event::ApplicationChange(current_application));
                    }
                    _ => {}
                }
            }
        }
    }

    fn watch_target_key_inputs(&self) -> Vec<domain::values::KeyInput<XKeySymbol, XModifier>> {
        self.watch_target_key_inputs.clone()
    }
}

struct XKeyPresser {
    state: Rc<Mutex<XState>>,
}

impl domain::interpreter::KeyPresser<XKeySymbol, XModifier> for XKeyPresser {
    fn press(&self, key_input: domain::values::KeyInput<XKeySymbol, XModifier>) {
        unsafe {
            let state = self.state.lock().unwrap();

            let mut focused_window = 0;
            let mut focus_state = 0;
            xlib::XGetInputFocus(state.display, &mut focused_window, &mut focus_state);

            let modifier_bits = key_input
                .modifiers()
                .to_vec()
                .into_iter()
                .fold(xlib::AnyModifier, |bits, modifier| {
                    bits | modifier.raw_value()
                });
            let key_event = xlib::XKeyEvent {
                type_: xlib::KeyPress,
                serial: 0,
                send_event: xlib::True,
                display: state.display,
                window: focused_window,
                root: xlib::XDefaultRootWindow(state.display),
                subwindow: 0,
                time: xlib::CurrentTime,
                x: 1,
                y: 1,
                x_root: 1,
                y_root: 1,
                state: modifier_bits,
                keycode: xlib::XKeysymToKeycode(state.display, key_input.key().raw_value()) as u32,
                same_screen: xlib::True,
            };
            let event = xlib::XEvent { key: key_event };

            xlib::XSendEvent(
                state.display,
                focused_window,
                xlib::True,
                xlib::KeyPressMask,
                &mut (event as xlib::XEvent),
            );
        }
    }
}

fn main() {
    let input_a = domain::values::KeyInput::new(
        domain::values::Key::new(0x61),
        domain::values::Modifiers::new(vec![]),
    );
    let input_b = domain::values::KeyInput::new(
        domain::values::Key::new(0x62),
        domain::values::Modifiers::new(vec![]),
    );

    unsafe {
        let display = xlib::XOpenDisplay(null());
        let window = xlib::XDefaultRootWindow(display);
        let state: Rc<Mutex<XState>> = Rc::new(Mutex::new(XState {
            display: display,
            window: window,
        }));
        let event_source = XEventSource {
            state: state.clone(),
            watch_target_key_inputs: vec![input_a.clone()], // TODO
        };
        let key_presser = XKeyPresser {
            state: state.clone(),
        };
        let current_application = {
            let state = state.lock().unwrap();
            state.fetch_current_application()
        };
        let interpreter: domain::interpreter::Interpreter<
            XKeyPresser,
            XAction,
            XKeySymbol,
            XModifier,
            XAppIdentifier,
        > = domain::interpreter::Interpreter {
            current_application: current_application,
            global_remaps: vec![domain::interpreter::Remap {
                from: input_a,
                to: input_b,
            }], // TODO
            global_exec_actions: vec![],                   // TODO
            remaps_for_application: BTreeMap::new(),       // TODO
            exec_actions_for_application: BTreeMap::new(), // TODO
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
    }
}
