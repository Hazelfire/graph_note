
pub mod database;
pub mod application;
pub mod view;
pub mod model;

use application::Application;


pub struct ViewModel {
    cursor_x: i32,
    cursor_y: i32,
    buffer: String,
    exiting: bool
}

pub enum Key {
    Right,
    Left,
    Up,
    Down,
    Escape,
    Other,
    Character(char),
}

fn subscribe(model: &mut Application, key: Key){
    model.send_key(key);
}

fn view_application(view_model: ViewModel){
    ncurses::clear();
    ncurses::mv(view_model.cursor_x, view_model.cursor_y);
    ncurses::addstr(&view_model.buffer);
    ncurses::refresh();
}

fn char_to_key(ch: i32) -> Key{
    use ncurses::*;
    use Key::*;
    match std::char::from_u32(ch as u32){
        Some(x) => {
            return Character(x);
        }
        None => {
            match ch {
                KEY_RIGHT => Right,
                KEY_LEFT => Left,
                KEY_UP => Up,
                KEY_DOWN => Down,
                27 => Escape,
                _ => Other
            }
        }
    }
}

fn start_application(db: database::Database){
    ncurses::initscr();
    ncurses::cbreak();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    let mut model = Application::init(db);

    loop {
        let view_model = view::view(&model);     
        if view_model.exiting {
            break;
        }
        view_application(view_model);
        let ch = ncurses::getch();
        subscribe(&mut model, char_to_key(ch));
    }

    ncurses::endwin();
}

pub fn start(filename: String){
    let dbr = database::Database::new(&filename);
    match dbr {
        Ok(db) => {
            start_application(db);
        }
        Err(_) => {
            println!("Could not load database");
        }
    }
}
