use speculate::speculate;

speculate! {
  describe "State" {
    describe "#run" {
      context "when ApplicationChanged event occured" {
        before {
          use mapper::*;
          use mapper::mock::*;

          let executor = MockShellCommandExecutor::new();
          let keybind_for_focus = MockKeyBindForFocus::new(
            maplit::btreemap!{},
            maplit::btreemap!{},
          );
          let possible_keyinput_finder = mapper::PossibleKeyinputFinder::new(maplit::btreemap!{});
          let key_handler = MockKeyHandler::new();

          let event_source = MockEventSource::new(vec![
            Event::ApplicationChanged { next_application: Some(mapper::Application::new("next_app".to_string())) }
          ]);

          let mut state = State::new(
            keybind_for_focus.clone(),
            possible_keyinput_finder,
            event_source.clone(),
            key_handler.clone(),
            executor.clone()
          );
        }

        it "updates application" {
          assert_eq!(state.application.is_none(), true);
          state.run();
          assert_eq!(state.application, Some(mapper::Application::new("next_app".to_string())))
        }

        it "ungrab and grab keys" {
          state.run();
          assert_eq!(*event_source.ungrabbed_count.lock().unwrap(), 1);
          assert_eq!(event_source.grabbed_keys.lock().unwrap().len(), 2); // 起動時に一回、application changedのときに一回
        }
      }

      context "when KeyPressed event occured, and keybind exists" {
        before {
          use mapper::*;
          use mapper::mock::*;

          let executor = MockShellCommandExecutor::new();
          let possible_keyinput_finder = mapper::PossibleKeyinputFinder::new(maplit::btreemap!{});
          let key_handler = MockKeyHandler::new();

          let from = KeyInput::of(String::from("a"), vec![String::from("Alt")]);
          let to = KeyInput::of(String::from("b"), vec![String::from("Control")]);

          let keybind_for_focus = MockKeyBindForFocus::new(
            maplit::btreemap!{
              (Focus::NoFocus, from.clone()) => Action::Key { key_input: to.clone() }
            },
            maplit::btreemap!{},
          );
          let event_source = MockEventSource::new(vec![
            Event::KeyPressed { key_input: from.clone() }
          ]);

          let mut state = State::new(
            keybind_for_focus.clone(),
            possible_keyinput_finder,
            event_source.clone(),
            key_handler.clone(),
            executor.clone()
          );
        }

        it "converted key is pressed" {
          state.run();
          assert_eq!(
            key_handler.pressed_keys.lock().unwrap()[0].clone(),
            to
          );
        }
      }

      context "when KeyReleased event occured, and keybind exists" {
        before {
          use mapper::*;
          use mapper::mock::*;

          let executor = MockShellCommandExecutor::new();
          let possible_keyinput_finder = mapper::PossibleKeyinputFinder::new(maplit::btreemap!{});
          let key_handler = MockKeyHandler::new();

          let from = KeyInput::of(String::from("a"), vec![String::from("Alt")]);
          let to = KeyInput::of(String::from("b"), vec![String::from("Control")]);

          let keybind_for_focus = MockKeyBindForFocus::new(
            maplit::btreemap!{},
            maplit::btreemap!{
              (Focus::NoFocus, from.clone()) => Action::Key { key_input: to.clone() }
            },
          );
          let event_source = MockEventSource::new(vec![
            Event::KeyReleased { key_input: from.clone() }
          ]);

          let mut state = State::new(
            keybind_for_focus.clone(),
            possible_keyinput_finder,
            event_source.clone(),
            key_handler.clone(),
            executor.clone()
          );
        }

        it "converted key is released" {
          state.run();
          assert_eq!(
            key_handler.released_keys.lock().unwrap()[0].clone(),
            to
          );
        }
      }
    }
  }
}
