use std::{
    collections::hash_map,
    f32::consts::TAU,
    hash::{Hash, Hasher},
    num::NonZeroU8,
};

use crate::{
    geom::{EdgePos, HexEdge},
    puzzle::{Level, Puzzle},
    resources::Resources,
    util::{hexcolor, mouse_position_pixel},
    HEIGHT, WIDTH,
};

use super::{
    coord_to_px, far_px_to_edge, px_to_edge, StateGameplay, HEX_HEIGHT,
    HEX_WIDTH, PATH_MIN_DIST,
};

use hex2d::{Angle, Coordinate, Direction};
use macroquad::prelude::*;

impl StateGameplay {
    pub(super) fn draw_(&self) {
        let res = Resources::get();
        let level = &res
            .levels
            .get(self.level_idxs.0, self.level_idxs.1)
            .unwrap();

        self.draw_background(&res);

        let coords = {
            let mut range = Coordinate::new(0, 0)
                .range_iter(level.puzzle.radius() as i32)
                .map(|c| (c, coord_to_px(c)))
                .collect::<Vec<_>>();
            range.sort_unstable_by(|(_, a), (_, b)| a.y.total_cmp(&b.y));
            range
        };

        for (_, center) in coords.iter().copied() {
            let px = center.x - HEX_WIDTH / 2.0;
            let py = center.y - HEX_HEIGHT / 2.0;
            draw_texture(res.textures.wheat_hex, px, py, WHITE);
        }

        for (coord, center) in coords.iter().copied() {
            self.draw_junctions(&level.puzzle, &res, coord, center);
        }

        // Draw center dots on top
        for (coord, center) in coords.iter().copied() {
            if self.board.get_junction_count(coord) != 0 {
                draw_texture_ex(
                    res.textures.paths,
                    center.x - 2.0,
                    center.y - 2.0,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(0.0, 60.0, 4.0, 4.0)),
                        ..Default::default()
                    },
                )
            }
        }

        // Draw edge numbers
        for (marks, (dir, start, deltas)) in level.puzzle.marks().iter().zip([
            (Direction::XY, (-22.0, -2.0), (-6.0, 0.0)),
            (Direction::YZ, (8.0, 15.0), (3.0, 6.0)),
            (Direction::ZX, (8.0, -19.0), (3.0, -6.0)),
        ]) {
            // scan the flank
            draw_flank_numbers(marks, level, dir, start, deltas, &res);
        }

        for (idx, b) in [&self.b_check, &self.b_back, &self.b_help]
            .iter()
            .enumerate()
        {
            let sx = idx as f32 * 9.0;
            let sy = if b.mouse_hovering() { 9.0 } else { 0.0 };
            draw_texture_ex(
                res.textures.buttons,
                b.x(),
                b.y(),
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(sx, sy, 9.0, 9.0)),
                    ..Default::default()
                },
            );
        }
    }

    fn draw_background(&self, res: &Resources) {
        for cell_x in 0..WIDTH as u32 / 16 {
            for cell_y in 0..HEIGHT as u32 / 16 {
                let px = cell_x as f32 * 16.0;
                let py = cell_y as f32 * 16.0;

                let sx = hash((cell_x + 1, 0x1234, self.level_idxs)) % 48;
                let sy = hash((cell_y + 1, 0x5678, self.level_idxs)) % 48;
                let flip_x = hash((cell_y + 2, 0x7604)) % 2 == 0;
                let flip_y = hash((cell_x + 2, 0o7604)) % 2 == 0;
                let rotation = (hash((cell_x, cell_y, self.level_idxs)) % 4)
                    as f32
                    * 0.25
                    * TAU;
                draw_texture_ex(
                    res.textures.background,
                    px,
                    py,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(sx as _, sy as _, 16.0, 16.0)),
                        flip_x,
                        flip_y,
                        rotation,
                        ..Default::default()
                    },
                );
            }
        }
    }

    fn draw_junctions(
        &self,
        puzzle: &Puzzle,
        res: &Resources,
        coord: Coordinate,
        center: Vec2,
    ) {
        let mouse_edge = far_px_to_edge(mouse_position_pixel(), PATH_MIN_DIST);

        let edges = self.board.get_raw_paths(coord);
        for edge in [HexEdge::XY, HexEdge::ZY, HexEdge::ZX] {
            let edgepos = EdgePos::new_raw(coord, edge);
            let mouse_matches = if let Some(mouse_edge) = mouse_edge {
                mouse_edge == edgepos
                    && self.board.can_twiddle_path(&puzzle, mouse_edge)
            } else {
                false
            };
            let opacity = match (edges.contains(edge), mouse_matches) {
                (true, false) => Some(1.0),
                (true, true) => Some(
                    ((get_time() as f32 * 4.0).sin() * 0.5 + 0.5) * 0.2 + 0.8,
                ),
                (false, true) => Some(
                    ((get_time() as f32 * 4.0).sin() * 0.5 + 0.5) * 0.4 + 0.5,
                ),
                (false, false) => None,
            };

            if let Some(opacity) = opacity {
                let (sy, sw, sh, dx, dy) = match edge {
                    HexEdge::XY => (0.0, 34.0, 4.0, -1.0, -2.0),
                    HexEdge::ZY => (4.0, 18.0, 28.0, -1.0, -2.0),
                    HexEdge::ZX => (32.0, 18.0, 28.0, -17.0, -2.0),
                };
                draw_texture_ex(
                    res.textures.paths,
                    center.x + dx,
                    center.y + dy,
                    Color::new(1.0, 1.0, 1.0, opacity),
                    DrawTextureParams {
                        source: Some(Rect::new(0.0, sy, sw, sh)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn draw_flank_numbers(
    marks: &Vec<Vec<NonZeroU8>>,
    level: &Level,
    dir: Direction,
    start: (f32, f32),
    deltas: (f32, f32),
    res: &crate::resources::ResourcesRef,
) {
    'side: for (i, markset) in marks.iter().enumerate() {
        if markset.is_empty() {
            continue 'side;
        }
        // ... -2, -1, 0, 1, 2 ...
        let centered_idx = i as i32 - level.puzzle.radius() as i32;
        let side_center = Coordinate::new(0, 0)
            - (Coordinate::from(dir).scale(level.puzzle.radius() as i32));
        let offset = Coordinate::from(
            dir + if centered_idx > 0 {
                Angle::Right
            } else {
                Angle::Left
            },
        )
        .scale(centered_idx.abs());
        let anchor = side_center + offset;
        let anchorpos = coord_to_px(anchor) + Vec2::from(start);

        for (j, mark) in markset.iter().rev().enumerate() {
            let cx = anchorpos.x + j as f32 * deltas.0;
            let cy = anchorpos.y + j as f32 * deltas.1;

            let sx = mark.get() as f32 * 4.0;
            draw_texture_ex(
                res.textures.numbers,
                cx,
                cy,
                hexcolor(0x48cf87_ff),
                DrawTextureParams {
                    source: Some(Rect::new(sx, 0.0, 4.0, 4.0)),
                    ..Default::default()
                },
            );
        }
    }
}

fn hash<H: Hash>(h: H) -> u64 {
    let mut hasher = hash_map::DefaultHasher::default();
    h.hash(&mut hasher);
    hasher.finish()
}
