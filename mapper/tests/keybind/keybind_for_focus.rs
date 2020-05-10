use speculate::speculate;

use mapper::mock::MockKeyBind;
use mapper::{
  Application, Focus, IsKeyBindForFocus, Key, KeyBind, KeyBindForFocus, KeyInput, Modifiers,
};

speculate! {
  describe "KeyBindForFocus#pressed" {
    context "global and inapp keybinds exist" {
      before {
        let global_keybind = Box::new(MockKeyBind {
          from: "global".to_string(),
          execution: "global_execution".to_string()
        });
        let inapp_keybind: Box<dyn KeyBind<String, String, String>> = Box::new(MockKeyBind {
          from: "inapp".to_string(),
          execution: "inapp_execution".to_string()
        });
        let keybind_by_application = maplit::btreemap! {
          Application::new("app".to_string()) => inapp_keybind
        };
        let keybind_for_focus = KeyBindForFocus::new(
          global_keybind,
          keybind_by_application,
        );
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
  }

  describe "KeyBindForFocus#released" {
    context "global and inapp keybinds exist" {
      before {
        let global_keybind = Box::new(MockKeyBind {
          from: "global".to_string(),
          execution: "global_execution".to_string()
        });
        let inapp_keybind: Box<dyn KeyBind<String, String, String>> = Box::new(MockKeyBind {
          from: "inapp".to_string(),
          execution: "inapp_execution".to_string()
        });
        let keybind_by_application = maplit::btreemap! {
          Application::new("app".to_string()) => inapp_keybind
        };
        let keybind_for_focus = KeyBindForFocus::new(
          global_keybind,
          keybind_by_application,
        );
      }

      it "matches to global remap" {
        assert_eq!(
          keybind_for_focus.released(
            Focus::NoFocus,
            KeyInput::new(Key::new("global".to_string()), Modifiers::new(vec![]))
          ),
          Some(mapper::Action::Execution {
            execution: "released/global_execution".to_string(),
          })
        );
      }

      it "matches to inapp remap" {
        assert_eq!(
          keybind_for_focus.released(
            Focus::Focused { application: mapper::Application::new("app".to_string()) },
            KeyInput::new(Key::new("inapp".to_string()), Modifiers::new(vec![]))
          ),
          Some(mapper::Action::Execution {
            execution: "released/inapp_execution".to_string(),
          })
        );
      }

      // focusがあたっているとき、そのfocusに対応したremapが存在しなくても、globalにremapがあればそれにmatchする
      it "matches to global remap through focused application" {
        assert_eq!(
          keybind_for_focus.released(
            Focus::Focused { application: mapper::Application::new("app".to_string()) },
            KeyInput::new(Key::new("global".to_string()), Modifiers::new(vec![]))
          ),
          Some(mapper::Action::Execution {
            execution: "released/global_execution".to_string(),
          })
        );
      }

      it "does not match to nonexistent remap" {
        assert_eq!(
          keybind_for_focus.released(
            Focus::NoFocus,
            KeyInput::new(Key::new("nonexistent".to_string()), Modifiers::new(vec![]))
          ),
          None
        );
      }
    }
  }
}
