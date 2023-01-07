use gerrymander::Transition;
use macroquad::prelude::*;

use crate::{
    resources::Resources, states::GameState, util::mouse_position_pixel,
};

use super::{px_to_coord, px_to_edge, StateGameplay};

impl StateGameplay {
    pub(super) fn update_(&mut self) -> Transition<GameState> {
        let res = Resources::get();
        let level = &res.levels.levels[self.level_idx];

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_edge = px_to_edge(mouse_position_pixel());

            let set = *self.painting_path.get_or_insert_with(|| {
                let here = self.board.get_path(mouse_edge).unwrap_or_default();
                !here
            });
            self.board.set_path(&level.puzzle, mouse_edge, set);
        } else {
            self.painting_path = None;
        }

        Transition::None
    }
}
