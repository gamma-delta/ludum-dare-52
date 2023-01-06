use std::{
    ops::Deref,
    sync::{Mutex, MutexGuard},
};

use macroquad::{
    audio::{load_sound, Sound},
    prelude::*,
};

use crate::puzzle::Level;

pub struct Resources {
    pub textures: Textures,
    pub sounds: Sounds,
    pub levels: Levels,
}

impl Resources {
    pub async fn init() {
        let textures = Textures::init().await;

        let sounds = Sounds::init().await;

        let levels = Levels::init().await;

        let mut lock = THE_RESOURCES.lock().unwrap();
        *lock = Some(Resources {
            textures,
            sounds,
            levels,
        });
    }

    pub fn get() -> ResourcesRef {
        let lock = THE_RESOURCES.lock().expect("resources mutex was locked :(");
        ResourcesRef(lock)
    }
}

#[cfg(debug_assertions)]
const RESOURCES_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources");
#[cfg(not(debug_assertions))]
const RESOURCES_ROOT: &str = "./resources";

static THE_RESOURCES: Mutex<Option<Resources>> = Mutex::new(None);

pub struct ResourcesRef(MutexGuard<'static, Option<Resources>>);

impl Deref for ResourcesRef {
    type Target = Resources;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("assets must be filled")
    }
}

pub struct Textures {}

impl Textures {
    async fn init() -> Self {
        Self {}
    }
}

pub struct Sounds {}

impl Sounds {
    async fn init() -> Self {
        Self {}
    }
}

pub struct Levels {
    pub levels: Vec<Level>,
}

impl Levels {
    async fn init() -> Self {
        let file = load_string(&format!("{}/puzzles.json5", RESOURCES_ROOT))
            .await
            .unwrap();
        let levels = json5::from_str(&file).unwrap();

        Self { levels }
    }
}

async fn texture(path: &str) -> Texture2D {
    let tex =
        load_texture(&format!("{}/textures/{}.png", RESOURCES_ROOT, path))
            .await
            .unwrap();
    tex.set_filter(FilterMode::Nearest);
    tex
}

async fn sound(path: &str) -> Sound {
    load_sound(&format!("{}/sounds/{}.ogg", RESOURCES_ROOT, path))
        .await
        .unwrap()
}
