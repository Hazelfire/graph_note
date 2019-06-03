use super::Application;
use super::database::Database;
use super::ViewModel;
use super::application::ViewMode;
use crate::model::Model;
use std::vec::Vec;

pub fn view(app : &Application) -> ViewModel {
    return ViewModel {
        buffer: view_buffer(&app.mode, &app.database),
        cursor_x: app.cursor.x,
        cursor_y: app.cursor.y,
        exiting: app.exiting
    }
}

fn view_buffer(mode : &ViewMode, db: &Database) -> String{
    return match mode {
        ViewMode::Normal => String::from("Records:\n{}"),
        ViewMode::Model => model_mode(db),
    }
}

fn model_mode(db: &Database) -> String{
    return format!("Models\n{}",view_models(&db.models));
}

fn view_models(models: &Vec<Model>) -> String{
    if models.len() == 0 {
        return String::from("[]");
    }
    else {
        let mut buffer = String::from("");
        for model in models {
            buffer.push_str(&format!("- name: {}\n", &model.name.to_string()))
        }
        buffer
    }
}
