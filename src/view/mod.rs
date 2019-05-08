use super::database::Database;

mod normal;
mod model;
use std::cmp::max;
use super::ViewModel;
use super::Key;
use normal::NormalMode;
use model::ModelMode;

struct Cursor {
    pub x: i32,
    pub y: i32,
}

enum CursorMessage {
    Move(i32, i32),
    Nothing
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
                return CursorMessage::Nothing;
            }
        }
    }

    fn update(&mut self, message: CursorMessage) {
        match message {
            CursorMessage::Move(dx, dy) => {
                self.x = max(0, self.x + dx);
                self.y = max(0, self.y + dy);
            }
            CursorMessage::Nothing => {}
        }
    }
}

pub enum ViewMode {
    Normal,
    Model
}



pub struct Application {
    database: Database,
    view_mode: ViewMode,
    exiting: bool,
    cursor: Cursor,
    normal_mode: NormalMode,
    model_mode: ModelMode,
}


pub enum ApplicationMessage {
    Exit,
    ChangeMode(ViewMode),
    Nothing
}

impl Application {
    pub fn init(db: Database) -> Self {
        Self {
            database: db,
            view_mode: ViewMode::Normal,
            exiting: false,
            cursor: Cursor::init(),
            normal_mode: NormalMode::init(),
            model_mode: ModelMode::init(),
        }
    }

    pub fn subscribe(&mut self, key: &Key) -> ApplicationMessage{
        use ApplicationMessage::*;
        let action = self.cursor.subscribe(key);
        self.cursor.update(action);
        match self.view_mode {
            ViewMode::Normal => {
                self.normal_mode.update(self.normal_mode.subscribe(key))    
            }
            ViewMode::Model => {
                self.model_mode.update(self.model_mode.subscribe(key))    
            }
        }
        match key{
            Key::Character(x) => {
                match x {
                    'q' => Exit,
                    'm' => ChangeMode(ViewMode::Model), 
                    'n' => ChangeMode(ViewMode::Normal),
                    _ => Nothing,
                }
            }
            _ => Nothing
        }
    }

    pub fn update(&mut self, message: ApplicationMessage ) {
        use ApplicationMessage::*;
        match message {
            Exit => {
                self.exiting = true;
            }
            ChangeMode(mode) => {
                self.view_mode = mode
            }
            _ => {}
        }
    }

    pub fn view(&self) -> ViewModel {
        use ViewMode::*;
        let buffer = match self.view_mode {
            Normal => self.normal_mode.view(),
            Model => self.model_mode.view()
        };
        ViewModel {
            exiting: self.exiting,
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            buffer: buffer,
        }
    }
}
