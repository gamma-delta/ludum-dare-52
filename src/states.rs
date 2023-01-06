mod gameplay;
pub use gameplay::StateGameplay;

use enum_dispatch::enum_dispatch;
use gerrymander::Transition;

#[enum_dispatch]
pub trait GameStateDispatch: Sized {
    /// Good ol' 60hz update.
    fn update(&mut self) -> Transition<GameState>;

    fn draw(&self);
}

#[enum_dispatch(GameStateDispatch)]
pub enum GameState {
    Gameplay(StateGameplay),
}

impl GameState {
    pub fn start() -> Self {
        GameState::Gameplay(StateGameplay::new(0))
    }
}
