use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

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


pub fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn read_username_from_file_chained_shortcut() -> Result<String, io::Error> {
    let mut username = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}

pub fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


pub fn propagate_with_different_types() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}



