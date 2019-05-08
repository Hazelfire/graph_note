

use crate::database::Database;
use super::ViewMode;
use super::ViewType;
use super::View;

pub struct ModelMode<'a> {
    db: &'a Database,
    view: &'a View<'a>,
}

// Normal mode
impl<'a> ViewMode for ModelMode<'a> {
    fn onKey(&self, ch: &u32){
        match std::char::from_u32(*ch){
            Some(x) => {
                match x {
                    'q' => {
                        self.view.exit();
                    },
                    'n' => {
                        self.view.set_mode(ViewType::NORMAL);
                    }
                    _ => {}
                }        
            },
            None => {}
        }
    }

    fn onEnter(&self){
        self.view.set_buffer(&String::from("Welcome to Model mode!"));
    }
}
