use speculate::speculate;

speculate! {
  describe "CompositKeyBind" {
    context "when two keybind are composed" {
      before {
        use crate::mock::TestAction;
        use mapper::{Action, CompositKeyBind, Key, KeyBind, KeyInput, Modifiers};

        let keybind_1 = TestAction {
          from: "bind_1".to_string(),
          execution: "exec_1".to_string()
        };
        let keybind_2 = TestAction {
          from: "bind_2".to_string(),
          execution: "exec_2".to_string()
        };
        let composit_keybind = CompositKeyBind::new(
          vec![Box::new(keybind_1), Box::new(keybind_2)]
        );
      }

      it "match according to input" {
        assert_eq!(
          composit_keybind.pressed(&KeyInput::new(Key::new("bind_1".to_string()), Modifiers::new(vec![]))),
          Some(Action::Execution { execution: "pressed/exec_1".to_string() })
        );

        assert_eq!(
          composit_keybind.released(&KeyInput::new(Key::new("bind_2".to_string()), Modifiers::new(vec![]))),
          Some(Action::Execution { execution: "released/exec_2".to_string() })
        );

        assert_eq!(
          composit_keybind.pressed(&KeyInput::new(Key::new("nonexistent".to_string()), Modifiers::new(vec![]))),
          None
        );
      }
    }
  }
}
