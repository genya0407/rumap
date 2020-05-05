use speculate::speculate;

speculate! {
  describe "Keymap" {
    describe "#pressed" {
      context "when equivalent key_input is specified" {
        before {
          use mapper::{KeyInput, Key, Modifier, Modifiers, Keymap, KeyBind, Action};

          let keymap: Box<dyn KeyBind<String, String, String>> = Box::new(Keymap::new(
            KeyInput::new(
              Key::new("j".to_string()),
              Modifiers::new(
                vec![
                  Modifier::new("Ctrl".to_string()),
                ]
              )
            ),
            KeyInput::new(
              Key::new("Down".to_string()),
              Modifiers::new(vec![])
            )
          ));
        }

        it "maps to specified key_input" {
          assert_eq!(
            keymap.pressed(
              &KeyInput::new(
                Key::new("j".to_string()),
                Modifiers::new(
                  vec![
                    Modifier::new("Ctrl".to_string()),
                  ]
                )
              )
            ),
            Some(
              Action::Key {
                key_input: KeyInput::new(
                  Key::new("Down".to_string()),
                  Modifiers::new(vec![])
                )
              }
            )
          )
        }
      }

      context "when superset key_input is specified" {
        before {
          use mapper::{KeyInput, Key, Modifier, Modifiers, Keymap, KeyBind, Action};

          let keymap: Box<dyn KeyBind<String, String, String>> = Box::new(Keymap::new(
            KeyInput::new(
              Key::new("j".to_string()),
              Modifiers::new(
                vec![
                  Modifier::new("Ctrl".to_string()),
                ]
              )
            ),
            KeyInput::new(
              Key::new("Down".to_string()),
              Modifiers::new(vec![])
            )
          ));
        }

        it "maps to specified key_input merged" {
          assert_eq!(
            keymap.pressed(
              &KeyInput::new(
                Key::new("j".to_string()),
                Modifiers::new(
                  vec![
                    Modifier::new("Ctrl".to_string()),
                    Modifier::new("Shift".to_string()),
                  ]
                )
              )
            ),
            Some(
              Action::Key {
                key_input: KeyInput::new(
                  Key::new("Down".to_string()),
                  Modifiers::new(vec![
                    Modifier::new("Shift".to_string()),
                  ])
                )
              }
            )
          )
        }
      }

      context "when different key_input is specified" {
        before {
          use mapper::{KeyInput, Key, Modifier, Modifiers, Keymap, KeyBind};

          let keymap: Box<dyn KeyBind<String, String, String>> = Box::new(Keymap::new(
            KeyInput::new(
              Key::new("j".to_string()),
              Modifiers::new(
                vec![
                  Modifier::new("Ctrl".to_string()),
                ]
              )
            ),
            KeyInput::new(
              Key::new("Down".to_string()),
              Modifiers::new(vec![])
            )
          ));
        }

        it "does not match" {
          assert_eq!(
            keymap.pressed(
              &KeyInput::new(
                Key::new("j".to_string()),
                Modifiers::new(vec![])
              )
            ),
            None
          )
        }
      }
    }

    describe "#released" {
      context "when equivalent key_input is specified" {
        before {
          use mapper::{KeyInput, Key, Modifier, Modifiers, Keymap, KeyBind, Action};
    
          let keymap: Box<dyn KeyBind<String, String, String>> = Box::new(Keymap::new(
            KeyInput::new(
              Key::new("j".to_string()),
              Modifiers::new(
                vec![
                  Modifier::new("Ctrl".to_string()),
                ]
              )
            ),
            KeyInput::new(
              Key::new("Down".to_string()),
              Modifiers::new(vec![])
            )
          ));
        }
    
        it "maps to specified key_input" {
          assert_eq!(
            keymap.released(
              &KeyInput::new(
                Key::new("j".to_string()),
                Modifiers::new(
                  vec![
                    Modifier::new("Ctrl".to_string()),
                  ]
                )
              )
            ),
            Some(
              Action::Key {
                key_input: KeyInput::new(
                  Key::new("Down".to_string()),
                  Modifiers::new(vec![])
                )
              }
            )
          )
        }
      }
    
      context "when superset key_input is specified" {
        before {
          use mapper::{KeyInput, Key, Modifier, Modifiers, Keymap, KeyBind, Action};
    
          let keymap: Box<dyn KeyBind<String, String, String>> = Box::new(Keymap::new(
            KeyInput::new(
              Key::new("j".to_string()),
              Modifiers::new(
                vec![
                  Modifier::new("Ctrl".to_string()),
                ]
              )
            ),
            KeyInput::new(
              Key::new("Down".to_string()),
              Modifiers::new(vec![])
            )
          ));
        }
    
        it "maps to specified key_input merged" {
          assert_eq!(
            keymap.released(
              &KeyInput::new(
                Key::new("j".to_string()),
                Modifiers::new(
                  vec![
                    Modifier::new("Ctrl".to_string()),
                    Modifier::new("Shift".to_string()),
                  ]
                )
              )
            ),
            Some(
              Action::Key {
                key_input: KeyInput::new(
                  Key::new("Down".to_string()),
                  Modifiers::new(vec![
                    Modifier::new("Shift".to_string()),
                  ])
                )
              }
            )
          )
        }
      }
    
      context "when different key_input is specified" {
        before {
          use mapper::{KeyInput, Key, Modifier, Modifiers, Keymap, KeyBind};
    
          let keymap: Box<dyn KeyBind<String, String, String>> = Box::new(Keymap::new(
            KeyInput::new(
              Key::new("j".to_string()),
              Modifiers::new(
                vec![
                  Modifier::new("Ctrl".to_string()),
                ]
              )
            ),
            KeyInput::new(
              Key::new("Down".to_string()),
              Modifiers::new(vec![])
            )
          ));
        }
    
        it "does not match" {
          assert_eq!(
            keymap.released(
              &KeyInput::new(
                Key::new("j".to_string()),
                Modifiers::new(vec![])
              )
            ),
            None
          )
        }
      }
    }
  }
}
