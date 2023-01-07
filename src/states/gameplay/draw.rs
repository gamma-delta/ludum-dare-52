use crate::{
    geom::{EdgePos, HexEdge},
    resources::Resources,
    util::mouse_position_pixel,
    HEIGHT, WIDTH,
};

use super::{
    coord_to_px, px_to_edge, StateGameplay, BOARD_CENTER_X, BOARD_CENTER_Y,
    HEX_HEIGHT, HEX_SPAN_X, HEX_SPAN_Y, HEX_WIDTH,
};

use hex2d::{Coordinate, IntegerSpacing};
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

        let mouse_edge = px_to_edge(mouse_position_pixel());
        for (coord, center) in coords.iter().copied() {
            let edges = self.board.get_paths(coord);
            for edge in [HexEdge::XY, HexEdge::ZY, HexEdge::ZX] {
                let edgepos = EdgePos::new_raw(coord, edge);
                let opacity = if edges.contains(edge) {
                    if mouse_edge == edgepos {
                        Some(
                            ((get_time() as f32).sin() * 0.5 + 0.5) * 0.2 + 0.8,
                        )
                    } else {
                        Some(1.0)
                    }
                } else if mouse_edge == edgepos {
                    Some(((get_time() as f32).sin() * 0.5 + 0.5) * 0.4 + 0.5)
                } else {
                    None
                };

                if let Some(opacity) = opacity {
                    let (sx, sw) = match edge {
                        HexEdge::XY => (0.0, 36.0),
                        HexEdge::ZY => (36.0, 28.0),
                        HexEdge::ZX => (54.0, 28.0),
                    };
                    draw_texture_ex(
                        res.textures.paths,
                        center.x - 1.0,
                        center.y - 1.0,
                        Color::new(1.0, 1.0, 1.0, opacity),
                        DrawTextureParams {
                            source: Some(Rect::new(sx, 0.0, sw, 28.0)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}
