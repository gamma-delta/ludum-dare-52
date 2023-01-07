#![feature(array_methods)]
#![feature(macro_metavar_expr)]

mod button;
mod geom;
mod puzzle;
mod resources;
mod states;
mod text;
mod util;

use gerrymander::StateMachine;
use resources::Resources;
use states::{GameState, GameStateDispatch};

use macroquad::prelude::{coroutines::start_coroutine, *};
use util::wh_deficit;

const WIDTH: f32 = 320.0;
const HEIGHT: f32 = 240.0;
const ASPECT_RATIO: f32 = WIDTH / HEIGHT;

fn window_conf() -> Conf {
    Conf {
        window_title: if cfg!(debug_assertions) {
            concat!(env!("CARGO_CRATE_NAME"), " v", env!("CARGO_PKG_VERSION"))
        } else {
            "Crop Circles"
        }
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let canvas = render_target(WIDTH as u32, HEIGHT as u32);
    canvas.texture.set_filter(FilterMode::Nearest);

    load_and_anim(canvas).await;

    let mut states = StateMachine::new(GameState::start());

    loop {
        let trans = states.active_mut().update();
        states.apply(trans).unwrap();

        set_camera(&Camera2D {
            render_target: Some(canvas),
            zoom: vec2(
                (WIDTH as f32).recip() * 2.0,
                (HEIGHT as f32).recip() * 2.0,
            ),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            ..Default::default()
        });
        clear_background(WHITE);

        states.active().draw();

        set_default_camera();
        clear_background(BLACK);

        // Figure out the drawbox.
        // these are how much wider/taller the window is than the content
        let (width_deficit, height_deficit) = wh_deficit();
        draw_texture_ex(
            canvas.texture,
            width_deficit / 2.0,
            height_deficit / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    screen_width() - width_deficit,
                    screen_height() - height_deficit,
                )),
                ..Default::default()
            },
        );

        next_frame().await
    }
}

async fn load_and_anim(canvas: RenderTarget) {
    let coro = start_coroutine(Resources::init());

    while !coro.is_done() {
        set_camera(&Camera2D {
            render_target: Some(canvas),
            zoom: vec2(
                (WIDTH as f32).recip() * 2.0,
                (HEIGHT as f32).recip() * 2.0,
            ),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            ..Default::default()
        });
        clear_background(GREEN);

        draw_text("Loading!", 20.0, 20.0, 16.0, BLACK);

        set_default_camera();
        clear_background(BLACK);

        // Figure out the drawbox.
        // these are how much wider/taller the window is than the content
        let (width_deficit, height_deficit) = wh_deficit();
        draw_texture_ex(
            canvas.texture,
            width_deficit / 2.0,
            height_deficit / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    screen_width() - width_deficit,
                    screen_height() - height_deficit,
                )),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
