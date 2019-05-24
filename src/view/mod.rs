use super::Application;
use super::ViewModel;
use super::model::ViewMode;

pub fn view(app : &Application) -> ViewModel {
    return ViewModel {
        buffer: view_buffer(&app.mode),
        cursor_x: app.cursor.x,
        cursor_y: app.cursor.y,
        exiting: app.exiting
    }
}

fn view_buffer(mode : &ViewMode) -> String{
    return match mode {
        ViewMode::Normal => String::from("Welcome to view mode!"),
        ViewMode::Model => String::from("Welcome to model mode!")
    }
}
