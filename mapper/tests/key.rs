mod key_input {
  mod match_to {
    use mapper::key::*;

    #[test]
    fn match_if_equal() {
      assert_eq!(
        KeyInput::new(Key::new("hoge"), Modifiers::new(vec![Modifier::new("a")])).match_to(
          &KeyInput::new(Key::new("hoge"), Modifiers::new(vec![Modifier::new("a")]))
        ),
        Matching::Remain(Modifiers::new(vec![]))
      )
    }

    #[test]
    fn match_if_subset() {
      assert_eq!(
        KeyInput::new(Key::new("hoge"), Modifiers::new(vec![Modifier::new("a")])).match_to(
          &KeyInput::new(
            Key::new("hoge"),
            Modifiers::new(vec![Modifier::new("a"), Modifier::new("b")])
          )
        ),
        Matching::Remain(Modifiers::new(vec![Modifier::new("b")]))
      )
    }

    #[test]
    fn not_match_if_not_subset() {
      assert_eq!(
        KeyInput::new(
          Key::new("hoge"),
          Modifiers::new(vec![Modifier::new("a"), Modifier::new("b")])
        )
        .match_to(&KeyInput::new(
          Key::new("hoge"),
          Modifiers::new(vec![Modifier::new("a")])
        )),
        Matching::Unmatched
      )
    }

    #[test]
    fn not_match_if_key_different() {
      assert_eq!(
        KeyInput::new(Key::new("hoge"), Modifiers::new(vec![Modifier::new("a")])).match_to(
          &KeyInput::new(Key::new("fuga"), Modifiers::new(vec![Modifier::new("a")]))
        ),
        Matching::Unmatched
      )
    }
  }
}
