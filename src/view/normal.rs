use crate::Key;

pub struct NormalMode {
    buffer: String
}

pub enum NormalModeMessage {
    Nothing
}

impl NormalMode {
    pub fn init() -> Self {
        Self {buffer: String::from("Welcome! Press m to add models")}
    }

    pub fn subscribe(&self,key: &Key) -> NormalModeMessage{
        use NormalModeMessage::*;
        Nothing
    }

    pub fn update(&self, message: NormalModeMessage) {
        
    }
    pub fn view(&self) -> String{
        return self.buffer.clone();
    }
}
