use speculate::speculate;

use mapper::config::values;
use mapper::config::{IsParser, Parser};
use mapper::*;
use std::collections::BTreeMap;

speculate! {
  describe "build_possible_keyinput_finder" {
    context "when remap with combination modifiers is configured" {
      before {
        let config = serde_json::from_str(
          r#"
          {
            "remap": {
              "h": {
                "to": "Left",
                "with": ["Shift", "Mod2"]
              },
              "l": {
                "to": "Right",
                "with": ["Shift", "Mod2"]
              }
            },
            "in_app": {
              "app": {
                "c": {
                  "to": "x",
                  "with": ["Shift", "Mod2"]
                },
                "a": {
                  "to": "s",
                  "with": ["Shift", "Mod2"]
                }
              }
            }
          }
          "#
        ).unwrap();

        let possible_keyinput_finder = Parser::build(&mapper::mock::StringIntoDomain)
          .build_possible_keyinput_finder(config)
          .unwrap();
      }

      it "calculates all possible inapp keyinputs" {
        let mut inapp_keyinputs = possible_keyinput_finder.find(
          Focus::Focused { application: Application::new("app".to_string()) }
        );
        inapp_keyinputs.sort();

        // globalのやつも返す
        let mut expect = vec![
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string())])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string()), Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string())])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string()), Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("c".to_string()), Modifiers::new(vec![])),
          KeyInput::new(Key::new("c".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string())])),
          KeyInput::new(Key::new("c".to_string()), Modifiers::new(vec![Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("c".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string()), Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("a".to_string()), Modifiers::new(vec![])),
          KeyInput::new(Key::new("a".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string())])),
          KeyInput::new(Key::new("a".to_string()), Modifiers::new(vec![Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("a".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string()), Modifier::new("Mod2".to_string())])),
        ];
        expect.sort();

        assert_eq!(
          inapp_keyinputs,
          expect
        );
      }

      it "calculates all possible global keyinputs" {
        let mut global_keyinputs = possible_keyinput_finder.find(
          Focus::NoFocus
        );
        global_keyinputs.sort();

        let mut expect = vec![
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string())])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string()), Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string())])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![Modifier::new("Mod2".to_string())])),
          KeyInput::new(Key::new("l".to_string()), Modifiers::new(vec![Modifier::new("Shift".to_string()), Modifier::new("Mod2".to_string())])),
        ];
        expect.sort();

        assert_eq!(
          global_keyinputs,
          expect
        );
      }
    }

    context "when same key is specified for global and inapp" {
      before {
        let config = serde_json::from_str(
          r#"
          {
            "remap": {
              "h": {
                "to": "Left",
                "with": ["global"]
              }
            },
            "in_app": {
              "app": {
                "h": {
                  "to": "x",
                  "with": ["inapp"]
                }
              }
            }
          }
          "#
        ).unwrap();

        let possible_keyinput_finder = Parser::build(&mapper::mock::StringIntoDomain)
          .build_possible_keyinput_finder(config)
          .unwrap();
      }

      // FIXME ほんとはinappがglobalを上書きするようにしたい
      it "returns global and inapp keyinputs" {
        let mut result = possible_keyinput_finder.find(Focus::Focused { application: Application::new("app".to_string()) });
        result.sort();

        let mut expect = vec![
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("inapp".to_string())])),
          KeyInput::new(Key::new("h".to_string()), Modifiers::new(vec![Modifier::new("global".to_string())])),
        ];
        expect.sort();

        assert_eq!(result, expect);
      }
    }
  }

  describe "build_keybind_for_focus" {
    context "when global remap and inapp remap is configured" {
      before {
        let remaps = values::Remaps(maplit::btreemap! {
          values::KeyInput("global".to_string()) =>
          values::Action::Execution {
            execute: values::Execution("global_execution".to_string()),
          },
        });

        let remaps_app = values::Remaps(maplit::btreemap! {
          values::KeyInput("inapp".to_string()) =>
          values::Action::Execution {
            execute: values::Execution("inapp_execution".to_string()),
          },
        });

        let in_app: BTreeMap<values::Application, values::Remaps> = maplit::btreemap! {
          values::Application("app".to_string()) => remaps_app,
        };

        let config = values::Config {
          remap: remaps,
          in_app: in_app,
        };

        let keybind_for_focus = Parser::build(&mapper::mock::StringIntoDomain)
          .build_keybind_for_focus(config)
          .unwrap();
      }

      it "matches to global remap" {
        assert_eq!(
          keybind_for_focus.pressed(
            Focus::NoFocus,
            KeyInput::new(Key::new("global".to_string()), Modifiers::new(vec![]))
          ),
          Some(mapper::Action::Execution {
            execution: "pressed/global_execution".to_string(),
          })
        );
      }

      it "matches to inapp remap" {
        assert_eq!(
          keybind_for_focus.pressed(
            Focus::Focused { application: mapper::Application::new("app".to_string()) },
            KeyInput::new(Key::new("inapp".to_string()), Modifiers::new(vec![]))
          ),
          Some(mapper::Action::Execution {
            execution: "pressed/inapp_execution".to_string(),
          })
        );
      }

      // focusがあたっているとき、そのfocusに対応したremapが存在しなくても、globalにremapがあればそれにmatchする
      it "matches to global remap through focused application" {
        assert_eq!(
          keybind_for_focus.pressed(
            Focus::Focused { application: mapper::Application::new("app".to_string()) },
            KeyInput::new(Key::new("global".to_string()), Modifiers::new(vec![]))
          ),
          Some(mapper::Action::Execution {
            execution: "pressed/global_execution".to_string(),
          })
        );
      }

      it "does not match to nonexistent remap" {
        assert_eq!(
          keybind_for_focus.pressed(
            Focus::NoFocus,
            KeyInput::new(Key::new("nonexistent".to_string()), Modifiers::new(vec![]))
          ),
          None
        );
      }
    }

    context "when global remap and inapp remap have common target keyinput" {
      before {
        let global_remaps = values::Remaps(maplit::btreemap! {
          values::KeyInput("common_0".to_string()) =>
          values::Action::KeyInput {
            to: values::KeyInput("global_key_0".to_string()),
            with: None,
          },
        });
        let remaps_app_0 = values::Remaps(maplit::btreemap! {
          values::KeyInput("common_0".to_string()) =>
          values::Action::KeyInput {
            to: values::KeyInput("inapp_key_0".to_string()),
            with: None,
          },
        });
        let in_app: BTreeMap<values::Application, values::Remaps> = maplit::btreemap! {
          values::Application("app_0".to_string()) => remaps_app_0,
        };
        let config = values::Config {
          remap: global_remaps,
          in_app: in_app,
        };
        let keybind_for_focus = Parser::build(&mapper::mock::StringIntoDomain)
          .build_keybind_for_focus(config)
          .unwrap();
      }

      it "inapp overrides global" {
        assert_eq!(
          keybind_for_focus.pressed(
            Focus::Focused {
              application: Application::new("app_0".to_string())
            },
            KeyInput::new(Key::new("common_0".to_string()), Modifiers::new(vec![]))
          ),
          Some(Action::Key {
            key_input: KeyInput::new(Key::new("inapp_key_0".to_string()), Modifiers::new(vec![]))
          })
        );
      }
    }
  }
}
