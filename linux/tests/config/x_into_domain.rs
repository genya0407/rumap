use linux::config::keysyms;
use linux::config::XIntoDomain;
use mapper::config::IsIntoDomain;
use speculate::speculate;

speculate! {
  describe "XIntoDomain" {
    describe "#into_domain_application" {
      context "when specified config application" {
        it "returns domain application" {
          assert_eq!(
            XIntoDomain.into_domain_application(mapper::config::Application("app".to_string())).ok(),
            Some(mapper::Application::new("app".to_string()))
          );
        }
      }
    }

    describe "#into_domain_keyinput" {
      context "when specified only key" {
        it "returns keyinput with key and modifiers" {
          assert_eq!(
            XIntoDomain.into_domain_keyinput(mapper::config::KeyInput("Return".to_string())).ok(),
            Some(
              mapper::KeyInput::new(
                mapper::Key::new(keysyms::KEYNAME_TO_KEYSYM.get("Return").unwrap().clone()),
                mapper::Modifiers::new(vec![])
              )
            )
          );
        }
      }

      context "when specified modifiers and key" {
        it "returns keyinput with key and modifiers" {
          assert_eq!(
            XIntoDomain.into_domain_keyinput(mapper::config::KeyInput("Control-Shift-Return".to_string())).ok(),
            Some(
              mapper::KeyInput::new(
                mapper::Key::new(keysyms::KEYNAME_TO_KEYSYM.get("Return").unwrap().clone()),
                mapper::Modifiers::new(
                  vec![
                    mapper::Modifier::new(keysyms::MODIFIERNAME_TO_MASK.get("Control").unwrap().clone()),
                    mapper::Modifier::new(keysyms::MODIFIERNAME_TO_MASK.get("Shift").unwrap().clone()),
                  ]
                )
              )
            )
          );
        }
      }

      context "when specified invalid key" {
        it "returns error" {
          assert_eq!(
            XIntoDomain.into_domain_keyinput(mapper::config::KeyInput("nonexistent".to_string())).is_err(),
            true
          );
        }
      }

      context "when specified invalid modifier" {
        it "returns error" {
          assert_eq!(
            XIntoDomain.into_domain_keyinput(mapper::config::KeyInput("NonExistent-Return".to_string())).is_err(),
            true
          );
        }
      }

      context "when specified empty string" {
        it "returns error" {
          assert_eq!(
            XIntoDomain.into_domain_keyinput(mapper::config::KeyInput("NonExistent-Return".to_string())).is_err(),
            true
          );
        }
      }
    }

    // 型さえあっていれば自明な実装なので書かない
    // describe "#into_domain_action" {
    // }
    // describe "#into_domain_key" {
    // }
    // describe "#into_domain_modifier" {
    // }
  }
}
