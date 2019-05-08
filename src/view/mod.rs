use super::database::Database;

//mod normal;
//mod model;
use std::cmp::max;
use super::ViewModel;
use super::Key;

struct Cursor {
    pub x: i32,
    pub y: i32,
}

enum CursorMessage {
    Move(i32, i32),
    Empty
}


impl Cursor {
    fn init() -> Self{
        Self{x: 0, y: 0}
    }
    fn subscribe(&self, ch: &Key) -> CursorMessage{
        use Key::*;
        match ch {
            Right => {
                return CursorMessage::Move(1,0);
            },
            Left => {
                return CursorMessage::Move(-1,0);
            },
            Up => {
                return CursorMessage::Move(0,1);
            },
            Down => {
                return CursorMessage::Move(0,-1);
            },
            _ => {
                return CursorMessage::Empty;
            }
        }
    }

    fn update(&mut self, message: CursorMessage) {
        match message {
            CursorMessage::Move(dx, dy) => {
                self.x = max(0, self.x + dx);
                self.y = max(0, self.y + dy);
            }
            CursorMessage::Empty => {}
        }
    }

    fn view(&self) {

    }
}

pub enum ViewMode {
    NORMAL,
    MODEL
}



pub struct Application {
    buffer: String,
    database: Database,
    view_type: ViewMode,
    exiting: bool,
    cursor: Cursor,
}


pub enum ApplicationMessage {
    Exit,
    Nothing
}

impl Application {
    pub fn init(db: Database) -> Self {
        Self {
            buffer: String::from("Welcome! Press m to add models"),
            database: db,
            view_type: ViewMode::NORMAL,
            exiting: false,
            cursor: Cursor::init()
        }
    }

    pub fn subscribe(&mut self, key: &Key) -> ApplicationMessage{
        let action = self.cursor.subscribe(key);
        self.cursor.update(action);
        match key{
            Key::Character(x) => {
                match x {
                    'q' => ApplicationMessage::Exit,
                    _ => ApplicationMessage::Nothing,
                }
            }
            _ => ApplicationMessage::Nothing
        }
    }

    pub fn update(mut self, message: ApplicationMessage ) -> Self{
        match message {
            ApplicationMessage::Exit => {
                self.exiting = true;
                self
            }
            _ => self
        }
    }

    pub fn view(&self) -> ViewModel {
        ViewModel {
            exiting: self.exiting,
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            buffer: self.buffer.clone(),
        }
    }
}
