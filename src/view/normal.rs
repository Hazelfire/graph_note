use crate::database::Database;
use super::ViewMode;
use super::ViewType;
use super::View;

pub struct NormalMode<'a> {
    db: &'a Database,
    view: &'a View<'a>,
}

// Normal mode
impl<'a> ViewMode for NormalMode<'a> {
    fn onKey(&self, ch: &u32){
        match std::char::from_u32(*ch){
            Some(x) => {
                match x {
                    'q' => {
                        self.view.exit();
                    },
                    'm' => {
                        self.view.setMode(ViewType::MODEL);
                    }
                    _ => {}
                }        
            },
            None => {}
        }
    }

    fn onEnter(&self){
        if self.db.model_len() == 0 {
            self.view.set_buffer(&String::from("You have no models! Press m to open the model editor, or q to quit"));
        }
        else {
            self.view.set_buffer(&String::from("Welcome back!"));
        }
    }
}
