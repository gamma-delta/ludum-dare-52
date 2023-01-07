use crate::{
    geom::{EdgePos, HexEdge},
    resources::Resources,
    util::mouse_position_pixel,
};

use super::{
    coord_to_px, far_px_to_edge, px_to_edge, StateGameplay, HEX_HEIGHT,
    HEX_WIDTH, PATH_MIN_DIST,
};

use hex2d::Coordinate;
use macroquad::prelude::*;

impl StateGameplay {
    pub(super) fn draw_(&self) {
        let res = Resources::get();
        let level = &res.levels.levels[self.level_idx];

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

        let mouse_edge = far_px_to_edge(mouse_position_pixel(), PATH_MIN_DIST);
        for (coord, center) in coords.iter().copied() {
            let edges = self.board.get_raw_paths(coord);
            for edge in [HexEdge::XY, HexEdge::ZY, HexEdge::ZX] {
                let edgepos = EdgePos::new_raw(coord, edge);
                let opacity = if edges.contains(edge) {
                    if mouse_edge == Some(edgepos) {
                        Some(
                            ((get_time() as f32 * 4.0).sin() * 0.5 + 0.5) * 0.2
                                + 0.8,
                        )
                    } else {
                        Some(1.0)
                    }
                } else if mouse_edge == Some(edgepos) {
                    Some(
                        ((get_time() as f32 * 4.0).sin() * 0.5 + 0.5) * 0.4
                            + 0.5,
                    )
                } else {
                    None
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
}
