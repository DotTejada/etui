use crate::LENGTH;

pub enum CurrentMode {
    Normal,
    Insert,
    Exiting
}

pub struct App {
    pub input: String,
    pub current_mode: CurrentMode,
    pub cursorpos: (u16, u16)
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            current_mode: CurrentMode::Normal,
            cursorpos: (0 + 1, LENGTH + 1)
        }
    }
}
