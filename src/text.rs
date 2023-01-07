//! Utilities for rendering text.

use itertools::*;
use macroquad::prelude::*;

/// Number of printable characters in an ASCII charset (including the non-printing character).
pub const CHARACTER_COUNT: usize = 96;

/// Quick-and-dirty draw some text with the upper-left corner at the given position,
/// with one pixel of space between each line and each char.
pub fn draw_pixel_text(
    text: &str,
    cx: f32,
    cy: f32,
    align: TextAlign,
    color: Color,
    font: Texture2D,
) {
    let mut cursor_x = 0usize;
    let mut cursor_y = 0usize;

    let char_width = font.width() / CHARACTER_COUNT as f32;
    let char_height = font.height();

    let line_widths = text.lines().map(|s| s.len()).collect_vec();

    for c in text.bytes() {
        let slice_idx = match c {
            b' '..=b'~' => c - 0x20,
            b'\n' => {
                cursor_x = 0;
                cursor_y += 1;
                continue;
            }
            // otherwise just do the non-printing character
            _ => 127,
        };
        let sx = slice_idx as f32 * char_width;

        let offset_prop = match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => -0.5,
            TextAlign::Right => -1.0,
        };
        let offset =
            line_widths[cursor_y] as f32 * (char_width + 1.0) * offset_prop;

        let x = cx + cursor_x as f32 * (char_width + 1.0) + offset;
        let y = cy + cursor_y as f32 * (char_height + 1.0);

        draw_texture_ex(
            font,
            x.round(),
            y.round(),
            color,
            DrawTextureParams {
                source: Some(Rect::new(sx, 0.0, char_width, char_height)),
                ..Default::default()
            },
        );

        cursor_x += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
