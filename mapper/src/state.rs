use crate::*;

pub trait IsState {
  fn run(&mut self);
}

pub struct State<
  A: PartialEq + Eq + PartialOrd + Ord + Clone,
  K: PartialEq + Eq + std::fmt::Debug + PartialOrd + Ord + Clone,
  M: PartialOrd + Ord + std::fmt::Debug + Clone,
  C: std::fmt::Debug + Clone,
  KBFF: IsKeyBindForFocus<A, K, M, C>,
  ES: IsEventSource<K, M, A>,
  KH: IsKeyHandler<K, M>,
  SCE: IsShellCommandExecutor<C>,
> {
  pub application: Option<Application<A>>,
  shell_command_executor: SCE,
  event_source: ES,
  key_handler: KH,
  key_bind_for_focus: KBFF,
  possible_keyinput_finder: PossibleKeyinputFinder<A, K, M>,
  _c: std::marker::PhantomData<C>,
}

impl<
    A: PartialEq + Eq + PartialOrd + Ord + Clone,
    K: PartialEq + Eq + std::fmt::Debug + PartialOrd + Ord + Clone,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
    KBFF: IsKeyBindForFocus<A, K, M, C>,
    ES: IsEventSource<K, M, A>,
    KH: IsKeyHandler<K, M>,
    SCE: IsShellCommandExecutor<C>,
  > State<A, K, M, C, KBFF, ES, KH, SCE>
{
  pub fn run(&mut self) {
    self.event_source.grab_keys(self.watch_target_key_inputs());

    loop {
      match self.event_source.next() {
        Some(Event::ApplicationChanged { next_application }) => {
          self.application = next_application;
          self.event_source.ungrab_keys();
          self.event_source.grab_keys(self.watch_target_key_inputs());
        }
        Some(Event::KeyPressed { key_input }) => {
          log::info!("PRESS {:?}", key_input);
          if let Some(action) = self
            .key_bind_for_focus
            .pressed(self.focus(), key_input.clone())
          {
            match action {
              Action::Key {
                key_input: bound_key_input,
              } => self.key_handler.press_key(bound_key_input),
              Action::Execution { execution } => self.shell_command_executor.execute(execution),
            }
          }
        }
        Some(Event::KeyReleased { key_input }) => {
          log::info!("RELEASE {:?}", key_input);
          if let Some(action) = self
            .key_bind_for_focus
            .released(self.focus(), key_input.clone())
          {
            match action {
              Action::Key {
                key_input: bound_key_input,
              } => self.key_handler.release_key(bound_key_input),
              Action::Execution { execution } => self.shell_command_executor.execute(execution),
            }
          }
        }
        None => return,
      }
    }
  }
}

impl<
    A: PartialEq + Eq + PartialOrd + Ord + Clone,
    K: PartialEq + Eq + std::fmt::Debug + Clone + PartialOrd + Ord,
    M: PartialOrd + Ord + std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
    KBFF: IsKeyBindForFocus<A, K, M, C>,
    ES: IsEventSource<K, M, A>,
    KH: IsKeyHandler<K, M>,
    SCE: IsShellCommandExecutor<C>,
  > State<A, K, M, C, KBFF, ES, KH, SCE>
{
  pub fn new(
    key_bind_for_focus: KBFF,
    possible_keyinput_finder: PossibleKeyinputFinder<A, K, M>,
    event_source: ES,
    key_handler: KH,
    shell_command_executor: SCE,
  ) -> Self {
    Self {
      application: None,
      event_source: event_source,
      key_handler: key_handler,
      key_bind_for_focus: key_bind_for_focus,
      possible_keyinput_finder: possible_keyinput_finder,
      shell_command_executor: shell_command_executor,
      _c: std::marker::PhantomData,
    }
  }

  fn watch_target_key_inputs(&self) -> Vec<KeyInput<K, M>> {
    let keyinputs = self.possible_keyinput_finder.find(self.focus());
    log::trace!("watch_target_key_inputs: {:?}", keyinputs);
    keyinputs
  }

  fn focus(&self) -> Focus<A> {
    match self.application.clone() {
      Some(app) => Focus::Focused { application: app },
      None => Focus::NoFocus,
    }
  }
}
