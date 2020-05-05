use mapper::*;
use speculate::speculate;

speculate! {
  describe "XExecutionKeyBind" {
    describe "#pressed" {
      context "when specified equivalent key" {
        before {
          use linux::XExecutionKeyBind;

          let key_input = KeyInput::new(
            Key::new(100),
            Modifiers::new(vec![]),
          );

          let x_execution_keybind = XExecutionKeyBind::new(
            key_input.clone(),
            "some_command".to_string()
          );
        }

        it "returns Action" {
          assert_eq!(
            x_execution_keybind.pressed(&key_input).is_some(),
            true
          );
        }
      }

      context "when specified superset key" {
        before {
          use linux::XExecutionKeyBind;

          let x_execution_keybind = XExecutionKeyBind::new(
            KeyInput::new(
              Key::new(100),
              Modifiers::new(vec![]),
            ),
            "some_command".to_string()
          );
        }

        it "returns None" {
          assert_eq!(
            x_execution_keybind.pressed(
              &KeyInput::new(
                Key::new(100),
                Modifiers::new(
                  vec![
                    Modifier::new(10)
                  ]
                ),
              )
            ).is_none(),
            true
          );
        }
      }
    }


    describe "#released" {
      context "when specified equivalent key" {
        before {
          use linux::XExecutionKeyBind;

          let key_input = KeyInput::new(
            Key::new(100),
            Modifiers::new(vec![]),
          );

          let x_execution_keybind = XExecutionKeyBind::new(
            key_input.clone(),
            "some_command".to_string()
          );
        }

        it "returns None" {
          assert_eq!(
            x_execution_keybind.released(&key_input).is_none(),
            true
          );
        }
      }
    }
  }
}
