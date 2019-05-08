
extern crate ncurses;

pub mod database;
pub mod view;

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
    Other,
    Character(char),
}

fn view_application(view_model: ViewModel){
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
    let mut app = view::Application::init(db);
    let mut ch;
    loop {
        // View application
        let view_model = app.view();
        if view_model.exiting {
            break;
        }
        view_application(view_model);

        // Get key and update
        ch = ncurses::getch();
        let action = app.subscribe(&char_to_key(ch));
        app = app.update(action);
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

