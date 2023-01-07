use std::f32::consts::TAU;

use gerrymander::Transition;
use hex2d::Direction;
use macroquad::prelude::*;

use crate::{
    geom::EdgePos,
    resources::Resources,
    states::{
        gameplay::{coord_to_px, px_to_coord},
        GameState,
    },
    util::mouse_position_pixel,
};

use super::{px_to_edge, StateGameplay};

impl StateGameplay {
    pub(super) fn update_(&mut self) -> Transition<GameState> {
        let res = Resources::get();
        let level = &res.levels.levels[self.level_idx];

        if is_key_pressed(KeyCode::Space) {
            let edge = px_to_edge(mouse_position_pixel());
            let pos = px_to_coord(mouse_position_pixel());
            println!("{:?}\n{:?}\n======", pos, edge);
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_edge = px_to_edge(mouse_position_pixel());

            if self.board.can_twiddle_path(&level.puzzle, mouse_edge) {
                let set = *self.painting_path.get_or_insert_with(|| {
                    let here =
                        self.board.get_path(mouse_edge).unwrap_or_default();
                    !here
                });
                self.board.set_path(&level.puzzle, mouse_edge, set);
            }
        } else {
            self.painting_path = None;
        }

        Transition::None
    }
}
