use crate::fields::TextField;
//use std::collections::HashMap;
pub enum FieldType {
    String,
    Integer,
    Double,
    Map(Box<FieldType>),
    List(Box<FieldType>),
}

pub trait Node {

}

pub struct Model {
    pub name: TextField,
//    pub fields: HashMap<String, FieldType>
}

impl Model {
    pub fn new() -> Self {
        Self {
            name: TextField::new(String::from("default")),
        }
    }
}

pub struct List {
    pub elements: Vec<Box<dyn Node>>,
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_create_metamodel(){
    }
}
