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
    pub name: String,
    pub adding: bool,
//    pub fields: HashMap<String, FieldType>
}

impl Model {
    pub fn new() -> Self {
        Self {
            name: String::from("default"),
            adding: true
        }
    }
}

pub struct List {
    pub elements: Vec<Box<dyn Node>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_metamodel(){
        //let mut modelfields = HashMap::new();
        //modelfields.insert(String::from("name"), FieldType::String);
       // modelfields.insert(String::from("fields"), FieldType::Map(Box::new(FieldType::String)));
        let metamodel = Model {
            name: String::from("model"),
            adding: true,
 //           fields: modelfields,
        };
    }
}
