use super::Key;

use super::database::Database;
use crate::model::Model;

mod fields;
mod modelmode;

pub enum ViewMode {
    Normal,
    Model
}

pub struct Application {
    pub mode: ViewMode,
    pub cursor: Cursor,
    pub database: Database,
    pub exiting : bool,
}


impl Application {
    pub fn init(db: Database) -> Self {
        return Self{
            mode: ViewMode::Normal,
            database: db,
            cursor: Cursor::init(),
            exiting: false,
        }
    }

    pub fn send_key(&mut self, key: Key){

        match key {
            Key::Character('m') => match self.mode {
                ViewMode::Normal => self.mode = ViewMode::Model,
                ViewMode::Model => self.mode = ViewMode::Normal
            },
            Key::Character('a') => {
                self.database.models.push(Model::new())
            },
            _ => {}
        };

        self.exiting = match key { Key::Character(key) => key == 'q', _ => {false} };
        self.cursor.update(key);
    }
}


pub struct Cursor {
    pub x: i32,
    pub y: i32,
}

impl Cursor {
    fn init() -> Self{
        Self{x: 0, y: 0}
    }

    fn update(&mut self, key: Key) {
        use Key::*;
        match key {
            Right => {
                self.x += 1;
            },
            Left => {
                self.x -= 1;
            },
            Up => {
                self.y -= 1;
            },
            Down => {
                self.y += 1;
            },
            _ => {}
        }
    }
}

