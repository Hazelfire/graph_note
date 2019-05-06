extern crate sqlite;

use std::vec::Vec;
use std::collections::HashMap;

enum FieldType {
    STRING
}

pub struct Model {
    pub name: String,
    pub fields: HashMap<String, FieldType>
}

pub struct Database{
    pub models: Vec<Model>,
    pub connection: sqlite::Connection
}

trait DatabaseTrait {
    fn model_len() -> i32;
}

impl DatabaseTrait for Database {
    fn model_len() -> i32{
        return models.len()
    }
    
}

pub fn create(filename: &str) -> Result<Database, Option<String>>{
    match sqlite::open(filename){
        Ok(connection) =>{
            return Ok(Database {models: Vec::new(), connection: connection});
        },
        Err(error) => {
            return Err(error.message);
        }
    };
}
