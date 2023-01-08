mod draw;
mod update;

use std::f32::consts::TAU;

use gerrymander::Transition;
use hex2d::{Angle, Coordinate, Direction, IntegerSpacing};
use macroquad::prelude::{vec2, Mat2, Mat3, Vec2};

use crate::{
    button::Button,
    geom::{EdgePos, HexEdge},
    puzzle::Board,
    HEIGHT, WIDTH,
};

use super::{GameState, GameStateDispatch};

const HEX_WIDTH: f32 = 32.0;
const HEX_HEIGHT: f32 = 32.0;

const HEX_SPAN_X: i32 = 32;
const HEX_SPAN_Y: i32 = 24;

const BOARD_CENTER_X: f32 = WIDTH / 2.0;
const BOARD_CENTER_Y: f32 = HEIGHT / 2.0 - HEX_WIDTH as f32;

const PATH_MIN_DIST: f32 = 0.5;

const MAT_COORD2PX: Mat3 = Mat3::from_cols_array(&[
    HEX_SPAN_X as f32,
    0.0,
    0.0,
    //
    HEX_SPAN_X as f32 / 2.0,
    HEX_SPAN_Y as f32,
    0.0,
    //
    BOARD_CENTER_X,
    BOARD_CENTER_Y,
    1.0,
]);

pub struct StateGameplay {
    board: Board,
    level_idxs: (usize, usize),

    /// None for not painting, Some(x) for turning it on or off
    painting_path: Option<bool>,

    b_check: Button,
    b_back: Button,
    b_help: Button,

    frames: u64,
    check_state: CheckState,
}

impl StateGameplay {
    pub fn new(level_row: usize, level_col: usize) -> Self {
        let bx = 3.0;
        let by = 3.0;
        let b_check = Button::new(bx, by, 9.0, 9.0, None);
        let b_back = Button::new(bx, by + 10.0, 9.0, 9.0, None);
        let b_help = Button::new(bx, by + 20.0, 9.0, 9.0, None);

        Self {
            board: Board::new(),
            level_idxs: (level_row, level_col),
            painting_path: None,

            b_check,
            b_back,
            b_help,

            frames: 0,
            check_state: CheckState::Waiting,
        }
    }
}

impl GameStateDispatch for StateGameplay {
    fn update(&mut self) -> Transition<GameState> {
        self.update_()
    }

    fn draw(&self) {
        self.draw_();
    }
}

// https://github.com/gamma-delta/haxagon/blob/0131b392adb50b03d66eb18a0105694dd1deb713/src/modes/playing/mod.rs#L349
fn px_to_coord(px: Vec2) -> Coordinate {
    let tf = MAT_COORD2PX.inverse();
    let xz = tf.transform_point2(px);
    round_coord(xz.x, -xz.x - xz.y)
}

fn round_coord(xf: f32, yf: f32) -> Coordinate {
    let q = xf.round() as i32;
    let r = yf.round() as i32;
    let qf = xf - q as f32;
    let rf = yf - r as f32;
    if q.abs() > r.abs() {
        Coordinate::new(q + (qf + rf / 2.0).round() as i32, r)
    } else {
        Coordinate::new(q, r + (rf + qf / 2.0).round() as i32)
    }
}

fn coord_to_px(coord: Coordinate) -> Vec2 {
    MAT_COORD2PX
        .transform_point2(vec2(coord.x as f32, (-coord.x - coord.y) as f32))
}

// https://github.com/gamma-delta/hexlife/blob/99b5e182d8916b61b13e2f99da17841c9b4f6e69/viewer/src/main.rs#L66
fn px_to_edge(px: Vec2) -> EdgePos {
    far_px_to_edge(px, 0.0).unwrap()
}

fn far_px_to_edge(px: Vec2, tolerance: f32) -> Option<EdgePos> {
    let coord = px_to_coord(px);

    let ideal_pos = coord_to_px(coord);
    let delta = px - ideal_pos;
    if delta.x.abs() <= (1.0 - tolerance) * HEX_SPAN_X as f32 * 0.5
        && delta.y.abs() <= (1.0 - tolerance) * HEX_SPAN_Y as f32 * 0.5
    {
        return None;
    }

    let angle = delta.y.atan2(delta.x);
    let clean_angle = ((angle / TAU) * 6.0).round() as i32;
    let dir = Direction::from_int(clean_angle + 2);
    Some(EdgePos::new(coord, dir))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CheckState {
    Waiting,
    No(u32),
    Yes(u32),
}
