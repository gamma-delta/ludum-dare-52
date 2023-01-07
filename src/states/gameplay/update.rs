use gerrymander::Transition;
use macroquad::prelude::*;

use crate::{
    resources::Resources, states::GameState, util::mouse_position_pixel,
};

use super::{far_px_to_edge, px_to_edge, StateGameplay, PATH_MIN_DIST};

impl StateGameplay {
    pub(super) fn update_(&mut self) -> Transition<GameState> {
        let res = Resources::get();
        let level = &res.levels.levels[self.level_idx];

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(mouse_edge) =
                far_px_to_edge(mouse_position_pixel(), PATH_MIN_DIST)
            {
                if self.board.can_twiddle_path(&level.puzzle, mouse_edge) {
                    let set = *self.painting_path.get_or_insert_with(|| {
                        let here =
                            self.board.get_path(mouse_edge).unwrap_or_default();
                        !here
                    });
                    self.board.set_path(&level.puzzle, mouse_edge, set);
                }
            }
        } else {
            self.painting_path = None;
        }

        if is_key_pressed(KeyCode::Space) {
            let status = self.board.is_solved(&level.puzzle);
            println!("{:?}", status);
        }

        Transition::None
    }
}
