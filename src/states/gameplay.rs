use gerrymander::Transition;

use crate::puzzle::Board;

use super::{GameState, GameStateDispatch};

pub struct StateGameplay {
    board: Board,
    level_idx: usize,
}

impl StateGameplay {
    pub fn new(level_idx: usize) -> Self {
        Self {
            board: Board::new(),
            level_idx,
        }
    }
}

impl GameStateDispatch for StateGameplay {
    fn update(&mut self) -> Transition<GameState> {
        Transition::None
    }

    fn draw(&self) {}
}
