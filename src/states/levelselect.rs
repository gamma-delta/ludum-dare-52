use crate::{button::Button, resources::Resources};

pub struct StateLevelSelect {
    buttons: Vec<Button>,
}

impl StateLevelSelect {
    pub fn new() -> Self {
        let mut buttons = Vec::new();
        
        let res = Resources::get();
        for (row, levels) in res.levels.rows.iter().enumerate() {
            let bx = 60 + row * 
            
            buttons.push(CutsceneButton {
                button: todo!(),
                row,
            })
        }
    }
}

struct LevelButton {
    button: Button,
    row: usize,
    col: usize,
}

struct CutsceneButton {
    button: Button,
    row: usize,
}

enum AButton {
    Level(LevelButton),
    Cutscene(CutsceneButton),
}

impl AButton {
    fn get_button(&self) -> &Button {
        match self {
            AButton::Level(l) => &l.button,
            AButton::Cutscene(c) => &c.button,
        }
    }

    fn get_button_mut(&mut self) -> &mut Button {
        match self {
            AButton::Level(l) => &mut l.button,
            AButton::Cutscene(c) => &mut c.button,
        }
    }
}
