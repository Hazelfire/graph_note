use crate::Key;

pub struct TextField {
    buffer: String,
    active: bool,
}

impl TextField {
    pub fn send_key(&mut self, key: &Key){
        match key {
            Key::Character(ch) => {
                self.buffer.push(*ch)
            },
            Key::Backspace => {
                self.buffer.pop();
            }
            Key::Enter => {
                self.active = false;
            }
            _ => {}
        }
    }
    
    pub fn to_string(&self) -> String{
        return self.buffer.clone();
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn new(buffer: String) -> Self{
        return Self {
            buffer: buffer,
            active: true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_send_keys(){
        use crate::Key::*;
        let mut field = TextField::new(String::from(""));
        field.send_key(Character('c'));
        field.send_key(Character('a'));
        field.send_key(Character('t'));
        field.send_key(Escape);
        assert_eq!(field.to_string(), "cat"); 
    }
}
