extern crate sqlite;

use std::vec::Vec;
use super::model::Model;

pub enum FieldType {
    String,
    Integer,
    Double,
    Map(Box<FieldType>),
    List(Box<FieldType>),
}

/// Database structure
pub struct Database{
    pub models: Vec<Model>,
    pub connection: sqlite::Connection
}

impl Database {
    pub fn model_len(&self) -> usize{
        return self.models.len()
    }
    
    fn fill_models(connection: &sqlite::Connection) -> Vec<Model>{
        let mut re = Vec::new();
        connection.iterate("select name from sqlite_master where type='table' and name not like 'sqlite_%'", |pairs| {
            for &(_, value) in pairs.iter() {
                re.push(Model{name: String::from(value.unwrap()), adding: false});
            }
            true
        }).unwrap_or(());
        re
    }

    pub fn new(filename: &str) -> Result<Database, Option<String>> {
        match sqlite::open(filename){
            Ok(connection) =>{
                let re = Database {
                    models: Database::fill_models(&connection),
                    connection: connection,
                };
                return Ok(re);
            },
            Err(error) => {
                return Err(error.message);
            }
        }
    }

}

