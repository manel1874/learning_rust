use std::fs::File;
use std::io::{self, Read};

pub fn read_username_from_file() -> Result<String, io::Error> {

    /*
    The function returns a type Result<T, E>, where
    
    1. the generic T has been filled in with the concrete type String;
    2. the generic type E has been filled in with the concrete type io::Error.
    */


    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


