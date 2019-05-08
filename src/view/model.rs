use crate::Key;

pub struct ModelMode {
    buffer: String
}

pub enum ModelModeMessage {
    Nothing
}

impl ModelMode {
    pub fn init() -> Self {
        Self {buffer: String::from("Welcome to models!")}
    }

    pub fn subscribe(&self,key: &Key) -> ModelModeMessage{
        use ModelModeMessage::*;
        Nothing
    }

    pub fn update(&self, message: ModelModeMessage) {
        
    }
    pub fn view(&self) -> String{
        return self.buffer.clone();
    }
}
