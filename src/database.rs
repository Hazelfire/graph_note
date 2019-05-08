extern crate sqlite;

use std::vec::Vec;


pub struct Model {
    pub name: String,
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
    
    pub fn new(filename: &str) -> Result<Database, Option<String>> {
        match sqlite::open(filename){
            Ok(connection) =>{
                return Ok(Database {models: Vec::new(), connection: connection});
            },
            Err(error) => {
                return Err(error.message);
            }
        };
    }
}

#[cfg(test)]
mod tests{
    use tempfile::NamedTempFile;
    #[test]
    fn test_create(){
        let tmp = NamedTempFile::new().unwrap();
        let db = Database::new(tmp.path());

        assert_eq!(db.model_len(), 0);

    }
}
