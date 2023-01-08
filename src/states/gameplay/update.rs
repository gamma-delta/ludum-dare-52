use gerrymander::Transition;
use macroquad::prelude::*;

use crate::{
    resources::Resources, states::GameState, util::mouse_position_pixel,
};

use super::{
    far_px_to_edge, px_to_edge, CheckState, StateGameplay, PATH_MIN_DIST,
};

impl StateGameplay {
    pub(super) fn update_(&mut self) -> Transition<GameState> {
        let res = Resources::get();
        let level = &res
            .levels
            .get(self.level_idxs.0, self.level_idxs.1)
            .unwrap();

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

        if let CheckState::No(timer) | CheckState::Yes(timer) =
            &mut self.check_state
        {
            if *timer == 0 {
                if matches!(self.check_state, CheckState::Yes(_)) {
                    // TODO: pop
                } else {
                    self.check_state = CheckState::Waiting;
                }
            } else {
                *timer -= 1;
            }
        }

        if is_key_pressed(KeyCode::Space)
            || self.b_check.mouse_hovering()
                && is_mouse_button_down(MouseButton::Left)
        {
            let status = self.board.is_solved(&level.puzzle);
            self.check_state = match status {
                Err(_) => CheckState::No(100),
                Ok(()) => CheckState::Yes(120),
            };
        }

        for b in [&mut self.b_check, &mut self.b_back, &mut self.b_help] {
            b.post_update();
        }

        self.frames += 1;

        Transition::None
    }
}
