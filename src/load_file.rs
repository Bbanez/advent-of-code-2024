use std::env::current_dir;
use std::fs::File;
use std::io::{Error, Read};

pub fn load_file(file_name: &str) -> Result<String, Error> {
    let current_path = current_dir();
    match current_path {
        Ok(current_path) => {
            let file_path = format!("{}/files/{}", current_path.to_str().unwrap(), file_name);
            match File::open(&file_path) {
                Ok(f) => {
                    let mut file = f;
                    let mut content = String::new();
                    match file.read_to_string(&mut content) {
                        Ok(_) => Ok(content),
                        Err(e) => Err(e),
                    }
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}
